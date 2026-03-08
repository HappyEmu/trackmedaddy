# trackmedaddy

A command-line tool for [Everhour](https://everhour.com) time tracking. Start and stop timers on tasks using Linear ticket numbers.

## Installation

Requires [Rust](https://rustup.rs/) 1.85+.

Install directly from GitHub:

```sh
cargo install --git https://github.com/HappyEmu/trackmedaddy.git
```

Or clone and install locally:

```sh
git clone https://github.com/HappyEmu/trackmedaddy.git
cd trackmedaddy
cargo install --path .
```

This installs the `trackmedaddy` binary to `~/.cargo/bin/`.

## Setup

Get your API key from your Everhour account settings, then:

```sh
trackmedaddy login
# Enter your Everhour API key: ****
# Config saved to /Users/<you>/Library/Application Support/trackmedaddy/config.toml
```

The config path depends on your OS (uses the system config directory).

To remove your saved API key:

```sh
trackmedaddy logout
# Config removed: /Users/<you>/Library/Application Support/trackmedaddy/config.toml
```

## Usage

### Start a timer

```sh
trackmedaddy start ADM-13
# Found task: Weekly [ADM-13] (ev:250564033107246)
# Timer started (status: active)
```

The command searches Everhour tasks for the ticket number and starts a timer on the first match.

### Check timer status

```sh
trackmedaddy status
# Running: Weekly [ADM-13] (elapsed: 00:00:04, today: 00:54:00)

trackmedaddy status   # when no timer is running
# No timer running.
```

### Stop the current timer

```sh
trackmedaddy stop
# Timer stopped: Weekly [ADM-13] (session: 00:01:00, today: 00:54:00)
```

### Install AI agent skill

Teach your AI coding agent (Claude Code, Codex) how to use trackmedaddy:

```sh
trackmedaddy skill claude   # installs to ~/.claude/skills/trackmedaddy/SKILL.md
trackmedaddy skill codex    # installs to ~/.codex/instructions/trackmedaddy.md
```

After installing, your agent can use `/trackmedaddy` to manage timers.

## Error handling

```sh
# Not logged in
trackmedaddy start ADM-13
# Error: Could not read config file. Run `trackmedaddy login` to set up your API key.

# No matching task
trackmedaddy start NONEXISTENT-99
# Error: No tasks found matching "NONEXISTENT-99"
```
