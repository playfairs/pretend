use std::fs;
use std::io::{
  Read,
  Write,
};
use std::os::unix::net::UnixStream;
use std::path::{
  Path,
  PathBuf,
};
use std::process::{
  self,
  Command,
};
use std::thread;
use std::time::Duration;

use serde_json::Value;
use serde_json::json;

use crate::error::{
  Error,
  Result,
};
use crate::rpc::Activity;

pub struct RpcClient {
  stream: Option<UnixStream>,
  application_id: String,
}

impl Default for RpcClient {
  fn default() -> Self {
    Self::new()
  }
}

impl RpcClient {
  pub fn new() -> Self {
    Self {
      stream: None,
      application_id: String::new(),
    }
  }

  pub fn set_activity(&mut self, application_id: &str, activity: Activity) -> Result<()> {
    self.connect(application_id)?;
    let payload = json!({
        "cmd": "SET_ACTIVITY",
        "args": {
            "pid": process::id(),
            "activity": activity
        },
        "nonce": format!("activity-{}", process::id())
    });
    self.send(&payload)
  }

  pub fn clear_activity(&mut self, application_id: &str) -> Result<()> {
    self.connect(application_id)?;
    let payload = json!({
        "cmd": "SET_ACTIVITY",
        "args": {
            "pid": process::id(),
            "activity": null
        },
        "nonce": format!("clear-{}", process::id())
    });
    self.send(&payload)
  }

  fn connect(&mut self, application_id: &str) -> Result<()> {
    if self.stream.is_some() && self.application_id == application_id {
      return Ok(());
    }

    let socket_path = self.find_socket().or_else(|| {
      self.try_launch_discord();
      thread::sleep(Duration::from_secs(2));
      self.find_socket()
    }).ok_or_else(|| {
      Error::Rpc(
        "Discord IPC socket not found (is Discord running and exposing its IPC socket to this user/session?)"
          .into(),
      )
    })?;
    let stream = UnixStream::connect(&socket_path)?;
    let handshake = json!({
        "v": 1,
        "client_id": application_id
    });
    self.stream = Some(stream);
    self.application_id = application_id.to_string();
    self.send(&handshake)?;
    self.read_response()?;
    Ok(())
  }

  fn send(&mut self, payload: &Value) -> Result<()> {
    let stream = self
      .stream
      .as_mut()
      .ok_or_else(|| Error::Rpc("not connected".into()))?;
    let body = serde_json::to_vec(payload)?;
    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(&(body.len() as u32).to_le_bytes());
    frame.extend_from_slice(&body);
    stream.write_all(&frame)?;
    stream.flush()?;
    Ok(())
  }

  fn read_response(&mut self) -> Result<()> {
    let stream = self
      .stream
      .as_mut()
      .ok_or_else(|| Error::Rpc("not connected".into()))?;
    let mut length_bytes = [0_u8; 4];
    stream.read_exact(&mut length_bytes)?;
    let length = u32::from_le_bytes(length_bytes) as usize;
    let mut payload = vec![0_u8; length];
    stream.read_exact(&mut payload)?;
    let _ = serde_json::from_slice::<Value>(&payload)?;
    Ok(())
  }

  fn find_socket(&self) -> Option<PathBuf> {
    let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR");
    let home_dir = directories::BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf());

    let mut candidates = Vec::new();
    if let Ok(path) = std::env::var("DISCORD_IPC_PATH") {
      candidates.push(PathBuf::from(path));
    }
    if let Some(runtime_dir) = runtime_dir.as_deref() {
      candidates.push(PathBuf::from(runtime_dir).join("discord-ipc-0"));
      candidates.push(PathBuf::from(runtime_dir).join("discord-ipc-1"));
      candidates.extend(discover_discord_socket_candidates(Path::new(runtime_dir)));
    }
    if let Some(home_dir) = home_dir.as_deref() {
      candidates.push(home_dir.join(".config/discord/discord-ipc-0"));
      candidates.push(home_dir.join(".config/discordptb/discord-ipc-0"));
      candidates.push(home_dir.join(".config/discordcanary/discord-ipc-0"));
      candidates.push(home_dir.join(".local/share/discord/discord-ipc-0"));
      candidates
        .push(home_dir.join(".var/app/com.discordapp.Discord/config/discord/discord-ipc-0"));
      candidates
        .push(home_dir.join(".var/app/com.discordapp.Discord/config/discordptb/discord-ipc-0"));
      candidates
        .push(home_dir.join(".var/app/com.discordapp.Discord/config/discordcanary/discord-ipc-0"));
      candidates.push(home_dir.join("Library/Application Support/discord/discord-ipc-0"));
      candidates.push(home_dir.join("Library/Application Support/discordptb/discord-ipc-0"));
      candidates.push(home_dir.join("Library/Application Support/discordcanary/discord-ipc-0"));
      candidates.push(home_dir.join(".config/discord-ipc-0"));
      candidates.push(home_dir.join(".config/discord/discord-ipc-1"));
      candidates.push(home_dir.join(".local/share/discord/discord-ipc-1"));
      candidates.extend(discover_discord_socket_candidates(
        &home_dir.join(".config"),
      ));
      candidates.extend(discover_discord_socket_candidates(
        &home_dir.join("Library/Application Support"),
      ));
      candidates.extend(discover_discord_socket_candidates(
        &home_dir.join(".local/share"),
      ));
    }
    for search_root in [
      Path::new("/run/user"),
      Path::new("/var/run"),
      Path::new("/tmp"),
    ] {
      candidates.extend(discover_discord_socket_candidates(search_root));
    }

    find_socket_in_candidates(candidates)
  }
}

fn find_socket_in_candidates(candidates: impl IntoIterator<Item = PathBuf>) -> Option<PathBuf> {
  candidates.into_iter().find(|path| path.exists())
}

impl RpcClient {
  fn try_launch_discord(&self) -> bool {
    let candidates = discord_launch_commands();
    for launch in candidates {
      let mut command = Command::new(&launch.program);
      command.args(&launch.args);
      if command.spawn().is_ok() {
        return true;
      }
    }
    false
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LaunchCommand {
  program: String,
  args: Vec<String>,
}

fn discord_launch_commands() -> Vec<LaunchCommand> {
  let mut commands = vec![
    LaunchCommand {
      program: "xdg-open".into(),
      args: vec!["discord://".into()],
    },
    LaunchCommand {
      program: "gio".into(),
      args: vec!["launch".into(), "discord".into()],
    },
    LaunchCommand {
      program: "discord".into(),
      args: vec![],
    },
    LaunchCommand {
      program: "Discord".into(),
      args: vec![],
    },
    LaunchCommand {
      program: "discord-ptb".into(),
      args: vec![],
    },
    LaunchCommand {
      program: "discord-canary".into(),
      args: vec![],
    },
    LaunchCommand {
      program: "flatpak".into(),
      args: vec!["run".into(), "com.discordapp.Discord".into()],
    },
  ];

  if cfg!(target_os = "macos") {
    commands.insert(
      0,
      LaunchCommand {
        program: "open".into(),
        args: vec!["-a".into(), "Discord".into()],
      },
    );
  }

  commands
}

fn discover_discord_socket_candidates(root: &Path) -> Vec<PathBuf> {
  let Ok(entries) = fs::read_dir(root) else {
    return Vec::new();
  };

  let mut paths = Vec::new();
  for entry in entries.filter_map(|entry| entry.ok()) {
    let path = entry.path();
    if is_discord_socket_candidate(&path) {
      paths.push(path.clone());
    }
    if path.is_dir() {
      paths.extend(discover_discord_socket_candidates(&path));
    }
  }
  paths
}

fn is_discord_socket_candidate(path: &Path) -> bool {
  path
    .file_name()
    .and_then(|name| name.to_str())
    .is_some_and(|name| name.starts_with("discord-ipc-"))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use std::os::unix::net::UnixListener;
  use std::time::{
    SystemTime,
    UNIX_EPOCH,
  };

  #[test]
  fn prefers_existing_discord_socket() {
    let unique = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("t{unique}"));
    let socket_path = temp_dir.join("discord-ipc-0");

    fs::create_dir_all(&temp_dir).unwrap();
    let _listener = UnixListener::bind(&socket_path).unwrap();

    let result = find_socket_in_candidates(vec![temp_dir.join("missing"), socket_path.clone()]);
    assert_eq!(result, Some(socket_path));

    let _ = fs::remove_dir_all(&temp_dir);
  }

  #[test]
  fn discovers_socket_files_from_runtime_roots() {
    let unique = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("t{unique}"));
    let socket_path = temp_dir.join("discord-ipc-0");

    fs::create_dir_all(&temp_dir).unwrap();
    let _listener = UnixListener::bind(&socket_path).unwrap();

    let discovered = discover_discord_socket_candidates(&temp_dir);
    assert!(discovered.iter().any(|path| path == &socket_path));

    let _ = fs::remove_dir_all(&temp_dir);
  }

  #[test]
  fn discovers_nested_socket_files() {
    let unique = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("t{unique}"));
    let socket_path = temp_dir.join("discord-ipc-1");

    fs::create_dir_all(&temp_dir).unwrap();
    let _listener = UnixListener::bind(&socket_path).unwrap();

    let discovered = discover_discord_socket_candidates(&temp_dir);
    assert!(discovered.iter().any(|path| path == &socket_path));

    let _ = fs::remove_dir_all(&temp_dir);
  }

  #[test]
  fn includes_common_discord_launch_commands() {
    let commands = discord_launch_commands();
    assert!(commands.iter().any(|launch| launch.program == "discord"));
    assert!(commands.iter().any(|launch| launch.program == "flatpak"));
  }
}
