# everhour

A command-line tool for [Everhour](https://everhour.com) time tracking. Start and stop timers on tasks using Linear ticket numbers.

## Installation

Requires [Rust](https://rustup.rs/) 1.85+.

```sh
cargo install --path .
```

This installs the `everhour` binary to `~/.cargo/bin/`.

## Setup

Get your API key from your Everhour account settings, then:

```sh
everhour login
# Enter your Everhour API key: ****
# Config saved to /Users/<you>/Library/Application Support/everhour/config.toml
```

The config path depends on your OS (uses the system config directory).

## Usage

### Start a timer

```sh
everhour start ADM-13
# Found task: Weekly [ADM-13] (ev:250564033107246)
# Timer started (status: active)
```

The command searches Everhour tasks for the ticket number and starts a timer on the first match.

### Stop the current timer

```sh
everhour stop
# Timer stopped: Weekly [ADM-13] (session: 00:01:00, today: 00:54:00)
```

## Error handling

```sh
# Not logged in
everhour start ADM-13
# Error: Could not read config file. Run `everhour login` to set up your API key.

# No matching task
everhour start NONEXISTENT-99
# Error: No tasks found matching "NONEXISTENT-99"
```
