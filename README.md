# everhour

A command-line tool for [Everhour](https://everhour.com) time tracking. Start and stop timers on tasks using Linear ticket numbers.

## Installation

Requires [Rust](https://rustup.rs/) 1.85+.

```sh
cargo install --path .
```

This installs the `everhour` binary to `~/.cargo/bin/`.

## Setup

Get your API key from [Everhour > Settings > API](https://app.everhour.com/#/account/profile), then:

```sh
everhour login
# Enter your Everhour API key: ****
# Config saved to ~/.config/everhour/config.toml
```

## Usage

### Start a timer

```sh
everhour start TRG-80
# Found task: [TRG-80] Implement user authentication (ev:12345678)
# Timer started (status: active)
```

The command searches Everhour tasks for the ticket number and starts a timer on the first match.

### Stop the current timer

```sh
everhour stop
# Timer stopped. Duration: 01:23:45
```

## Error handling

```sh
# Not logged in
everhour start TRG-80
# Error: Could not read config file. Run `everhour login` to set up your API key.

# No matching task
everhour start NONEXISTENT-99
# Error: No tasks found matching "NONEXISTENT-99"
```
