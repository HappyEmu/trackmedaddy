---
name: trackmedaddy
description: Manage Everhour time tracking. Use when the user wants to start, stop, or check a timer for a Linear ticket. Triggers include "track time", "log time", "start working on", "what am I tracking", "stop timer", or any mention of tracking time on a ticket.
argument-hint: "[start|stop|status|login|logout] [TICKET]"
allowed-tools: Bash
---

Run `trackmedaddy` with the appropriate subcommand based on $ARGUMENTS.

If no arguments are provided, run `trackmedaddy status` to show the current timer.

## Commands

- `trackmedaddy start <TICKET>` — start a timer. `<TICKET>` is a Linear ticket ID (e.g. `TRG-80`, `ENG-123`).
- `trackmedaddy stop` — stop the current timer.
- `trackmedaddy status` — show the running timer.
- `trackmedaddy login` — set up the Everhour API key (interactive prompt).
- `trackmedaddy logout` — remove the stored API key.

Always show the command output to the user.

## Setup

If the command fails with a missing config or auth error, run `trackmedaddy login` to set up the API key.
