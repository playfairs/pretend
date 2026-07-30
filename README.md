# pretend

pretend is a Discord Rich Presence manager and spoofer for local Discord IPC. It lets you choose from a curated set of Discord applications, publish activities, and clear them again from the terminal.

## Features

- Manage Discord Rich Presence activities from the command line
- Search applications by name or aliases
- Store configuration in ~/.config/pretend
- Load default application definitions from data/applications.json
- Publish and clear activities over Discord IPC

## Installation

### Cargo

cargo build

### Nix

nix develop

## Usage

```bash
pretend list
pretend search minecraft
pretend start minecraft --details "Building" --state "Creative"
pretend stop
pretend config
```

## Configuration

The tool stores data in ~/.config/pretend:

```toml
[settings]
default_application = "minecraft"
last_application_id = "365975655608745985"
```

## Architecture

- pretend: CLI parsing, commands, and terminal output
- pretend-core: application database, config handling, RPC client, models, and utilities

## Nix

```bash
nix fmt .
nix develop
```
