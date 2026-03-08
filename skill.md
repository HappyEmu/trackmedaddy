---
name: trackmedaddy
description: Manage Everhour time tracking. Use when the user wants to start, stop, or check a timer for a Linear ticket.
disable-model-invocation: true
argument-hint: "[start|stop|status] [TICKET]"
allowed-tools: Bash
---

Run `trackmedaddy` with the appropriate subcommand based on $ARGUMENTS.

If no arguments are provided, run `trackmedaddy status` to show the current timer.

## Commands

- `trackmedaddy start <TICKET>` — start a timer. `<TICKET>` is a Linear ticket ID like `TRG-80`.
- `trackmedaddy stop` — stop the current timer.
- `trackmedaddy status` — show the running timer.

Always show the command output to the user.

## Setup

If the command fails with a missing config error, ask the user to run `trackmedaddy login` in their terminal to set up their API key.
