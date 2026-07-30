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
use std::process;

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

    let socket_path = self
      .find_socket()
      .ok_or_else(|| Error::Rpc("Discord IPC socket not found".into()))?;
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
    let mut candidates = Vec::new();
    if let Ok(path) = std::env::var("DISCORD_IPC_PATH") {
      candidates.push(PathBuf::from(path));
    }
    if let Some(runtime_dir) = std::env::var_os("XDG_RUNTIME_DIR") {
      candidates.push(PathBuf::from(runtime_dir).join("discord-ipc-0"));
    }
    if let Some(home) = directories::BaseDirs::new() {
      candidates.push(home.home_dir().join(".config/discord/discord-ipc-0"));
      candidates.push(home.home_dir().join(".config/discordptb/discord-ipc-0"));
      candidates.push(home.home_dir().join(".config/discordcanary/discord-ipc-0"));
      candidates.push(
        home
          .home_dir()
          .join("Library/Application Support/discord/discord-ipc-0"),
      );
      candidates.push(
        home
          .home_dir()
          .join("Library/Application Support/discordptb/discord-ipc-0"),
      );
      candidates.push(
        home
          .home_dir()
          .join("Library/Application Support/discordcanary/discord-ipc-0"),
      );
    }
    candidates.into_iter().find(|path| path.exists())
  }
}
