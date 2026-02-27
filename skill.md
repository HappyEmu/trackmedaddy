# Everhour CLI Usage

`everhour` is a CLI tool for Everhour time tracking. It starts and stops timers on tasks using Linear ticket numbers.

## Commands

### Login / Logout

```sh
everhour login    # prompts for API key, saves to config
everhour logout   # removes saved API key
```

### Start a timer

```sh
everhour start <TICKET>
```

`<TICKET>` is a Linear ticket identifier like `TRG-80` or `ADM-13`. The command searches Everhour tasks matching that ticket and starts a timer on the first result.

### Check status

```sh
everhour status
```

Shows the currently running timer with task name, elapsed time, and today's total. Prints "No timer running." if idle.

### Stop the timer

```sh
everhour stop
```

Stops the current timer and shows task name, session duration, and today's total.

## Typical workflow

```sh
everhour start TRG-80    # start working on a ticket
everhour status          # check how long you've been going
everhour stop            # done for now
```

## Config

The API key is stored at the OS config directory (e.g. `~/Library/Application Support/everhour/config.toml` on macOS). Get your key from your Everhour account settings.
