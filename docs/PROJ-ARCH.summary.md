# Project Architecture — Summary

## Overview

Shell utility for terminal tab management. Layered architecture: POSIX shared libraries + shell-specific adapters (Bash/Zsh).

## Components

- **lib/core.sh** -- Color/emoji mapping, urgency levels, tab state rendering
- **lib/terminal.sh** -- Terminal detection (10+ emulators), escape sequence abstraction
- **lib/history.sh** -- Tab ID generation, YAML event logging, search, reporting
- **lib/recording.sh** -- asciinema recording lifecycle
- **lib/todo.sh** -- Per-tab todo CRUD with provider pattern
- **shell/tabbing.zsh** -- Zsh adapter (1-based arrays, precmd hook)
- **shell/tabbing.bash** -- Bash adapter (0-based arrays, PROMPT_COMMAND hook)
- **bin/tabbing-init** -- Bootstrap: outputs `source` command for user's shell

## State

- **Runtime**: Environment variables (`TAB_TITLE`, `TAB_STATUS`, `TAB_ID`, etc.)
- **Persistent**: YAML files under `$XDG_STATE_HOME/tabbing/` (history, todos, recordings per tab)

## Key Decisions

- POSIX library layer for portability; shell-specific syntax only in adapters
- Environment variables for session state; YAML for persistence
- Per-tab isolation via unique TAB_ID
- No external dependencies (asciinema optional)
- Provider pattern for extensible todo backends
