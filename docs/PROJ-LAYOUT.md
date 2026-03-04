# Project Layout

Shell utility for managing terminal tab titles, status, todos, and recordings.
Supports Bash 4.0+ and Zsh 5.0+ with a shared POSIX library foundation.

```
tabbing-on/
├── bin/                            # Entry points & CLI wrappers
│   ├── tabbing-init                #   Shell bootstrapper — eval "$(tabbing-init bash|zsh)"
│   ├── demo-runner                #   Typewriter-style interactive demo runner
│   ├── _tabbing-wrapper            #   Shared setup: sources adapter + all libs, loads session
│   ├── _tabbing-commit             #   Side-effects helper: history, display, session save
│   ├── tabbing-on                  #   CLI: set/display tab title & status
│   ├── tabbing-status              #   CLI: update status
│   ├── tabbing-todo                #   CLI: manage todos (supports --list-pending, --export-switch)
│   ├── tabbing-report              #   CLI: time-in-state reports
│   ├── tabbing-history             #   CLI: search/browse history
│   ├── tabbing-recordings          #   CLI: manage recordings
│   ├── tabbing-info                #   CLI: full state dump
│   └── tabbing-clear               #   CLI: clear history/todos/recordings
├── lib/                            # POSIX-compatible shared libraries
│   ├── render.sh                   #   Render pipeline: emoji, color, display, title escape sequences
│   ├── core.sh                     #   Supplementary: emoji list, color list, help, YAML escape
│   ├── terminal.sh                 #   Terminal detection, badge, clear (non-render functions)
│   ├── history.sh                  #   Tab ID generation, YAML history tracking
│   ├── recording.sh               #   asciinema recording lifecycle
│   ├── session.sh                  #   Per-session state persistence (TAB_SESSION-keyed files)
│   └── todo.sh                     #   Per-tab todo management (provider pattern)
├── shell/                          # Shell-specific thin adapters
│   ├── tabbing.bash                #   Bash: sources render.sh, defines public functions, delegates to bin/
│   └── tabbing.zsh                 #   Zsh: sources render.sh, defines public functions, delegates to bin/
├── demo/                           # Demo scripts
│   └── showcase.demo               #   Interactive feature walkthrough
├── tests/                          # Test suites (empty)
├── docs/                           # Documentation
│   ├── PROJ-LAYOUT.md              #   This file
│   └── PROJ-LAYOUT.summary.md     #   Quick-reference tree
├── LICENSE                         # MIT (Copyright 2026 Keith Brings)
├── TODO.md                         # Roadmap & known limitations
├── script.md                       # Demo command reference
└── terminal-utils.zshrc            # Legacy shim — prefer tabbing-init
```

## Commands

After `eval "$(tabbing-init bash|zsh)"`:

| Command | Purpose |
|---------|---------|
| `tabbing-on [args]` | Set/display tab title, status, color, urgency, emoji |
| `tabbing-status` | Update status with emoji/urgency |
| `tabbing-todo` | Add/list/pick/done todo items |
| `tabbing-report` | ASCII/Mermaid reports of time-in-state |
| `tabbing-history` | Search/browse tab history |
| `tabbing-recordings` | Manage asciinema recordings |
| `tabbing-info` | Full state dump + file paths |

## Data Storage

XDG-compliant (`~/.local/state/tabbing/`):

```
~/.local/state/tabbing/
├── history/{TAB_ID}.yaml           # Title/status change log
├── todos/{TAB_ID}.yaml             # Todo items per tab
├── recordings/{TAB_ID}/*.cast      # asciinema recordings
└── sessions/{TAB_SESSION}.env      # Persisted env state for CLI wrappers
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `TAB_TITLE` | Tab title text |
| `TAB_STATUS` | Tab status text |
| `TAB_HIGHLIGHT` | Color name for title highlight |
| `TAB_URGENCY` | 0–5 (0=critical/red, 5=nominal/green) |
| `TAB_EMOJI` | Named emoji (overrides urgency dot) |
| `TAB_TERMINAL` | Detected terminal emulator |
| `TAB_ID` | Unique tab fingerprint (hex) |
| `TAB_SESSION` | Session fingerprint for state file scoping (hex) |
| `TABBING_ROOT` | Installation root directory |
