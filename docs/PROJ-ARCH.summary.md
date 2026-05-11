# Project Architecture — Summary

## Overview

Shell utility for terminal tab management. Layered architecture: POSIX shared libraries + shell-specific adapters (Bash/Zsh). Integrates with Claude Code IDE and Toggl Track for statusline display and time tracking.

## Components

- **lib/render.sh** -- Minimal render pipeline for prompt hook: title display, color/emoji validation
- **lib/core.sh** -- Color/emoji mapping, urgency levels, help, YAML escape
- **lib/terminal.sh** -- Terminal detection (10+ emulators), escape sequence abstraction
- **lib/history.sh** -- Tab ID generation, YAML event logging, search, reporting
- **lib/recording.sh** -- asciinema recording lifecycle
- **lib/session.sh** -- Per-session state persistence via TAB_SESSION-keyed env files
- **lib/todo.sh** -- Per-tab todo CRUD with provider pattern
- **lib/claude.sh** -- Claude Code IDE bridge via FIFO + state file
- **lib/toggl.sh** -- Toggl Track API integration for time entry lifecycle
- **bin/tabbing-claude-statusline** -- Claude Code statusline script (reads state file)
- **shell/tabbing.zsh** -- Zsh adapter (1-based arrays, precmd hook)
- **shell/tabbing.bash** -- Bash adapter (0-based arrays, PROMPT_COMMAND hook)
- **bin/tabbing-init** -- Bootstrap: outputs `source` command for user's shell

## State

- **Runtime**: Environment variables (`TAB_TITLE`, `TAB_STATUS`, `TAB_ID`, `TAB_SESSION`, etc.)
- **Persistent**: YAML files under `$XDG_STATE_HOME/tabbing/` (history, todos, recordings per tab; session env files)
- **Claude bridge**: FIFO pipe + flat state file + PID file per session under `$XDG_STATE_HOME/tabbing/`

## Key Decisions

- POSIX library layer for portability; shell-specific syntax only in adapters
- Render.sh is the only lib sourced at shell init (lightweight); other libs loaded on demand by bin/ scripts
- Environment variables for session state; YAML for persistence
- Per-tab isolation via unique TAB_ID; per-session isolation via TAB_SESSION
- No external dependencies (asciinema optional)
- Provider pattern for extensible todo backends
- Claude bridge uses FIFO to decouple render pipeline from IDE statusline
- Toggl integration is opt-in (requires TAB_TOGGL_TOKEN)
