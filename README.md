# pretend

pretend is a Discord Rich Presence manager and spoofer built in Rust. It loads application definitions, searches them by name or alias, and publishes Rich Presence activities through Discord IPC.

## Planned Features

- Start and stop Discord Rich Presence activities from the terminal
- Search applications by name or alias
- Load application definitions from data/applications.json
- Store configuration and cache under ~/.config/pretend
- Publish activities through local Discord IPC sockets

## Installation

### Cargo

```
cargo build
```

### Nix

```
nix develop
```

## Usage

```bash
pretend list
pretend search minecraft
pretend start minecraft --details "Building" --state "Creative"
pretend stop
pretend config
```

## Nix

```bash
nix fmt .
nix develop
```
