# Project Layout

Shell utility for managing terminal tab titles, status, todos, and recordings.
Supports Bash 4.0+ and Zsh 5.0+ with a shared POSIX library foundation.

```
tabbing-on/
├── bin/                            # Entry points
│   ├── tabbing-init                #   Shell bootstrapper — eval "$(tabbing-init bash|zsh)"
│   └── demo-runner                #   Typewriter-style interactive demo runner
├── lib/                            # POSIX-compatible shared libraries
│   ├── core.sh                     #   Colors, emoji, urgency, tab state rendering
│   ├── history.sh                  #   Tab ID generation, YAML history tracking
│   ├── recording.sh               #   asciinema recording lifecycle
│   ├── terminal.sh                 #   Terminal detection & escape sequences
│   └── todo.sh                     #   Per-tab todo management (provider pattern)
├── shell/                          # Shell-specific integrations
│   ├── tabbing.bash                #   Bash function definitions & PROMPT_COMMAND hook
│   └── tabbing.zsh                 #   Zsh function definitions & precmd hook
├── demo/                           # Demo scripts
│   └── showcase.demo               #   Interactive feature walkthrough
├── tests/                          # Test suites (empty)
├── docs/                           # Documentation
│   ├── PROJ-LAYOUT.md              #   This file
│   └── PROJ-LAYOUT.summary.md     #   Quick-reference tree
├── LICENSE                         # MIT (Copyright 2026 Keith Brings)
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
├── history/{TAB_ID}.yaml       # Title/status change log
├── todos/{TAB_ID}.yaml         # Todo items per tab
└── recordings/{TAB_ID}/*.cast  # asciinema recordings
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
| `TABBING_ROOT` | Installation root directory |
