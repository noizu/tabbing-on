# Project Layout

Terminal tab title/status/todo/recording manager. Two parallel implementations —
the original pure-shell version (`shell-impl/`, Bash 4.0+/Zsh 5.0+ on a POSIX lib
foundation) and the primary Rust multi-call binary (`rust/`, v0.2.0) — plus an
Ink/React TUI for voice-memo-to-ticket capture (`ink-plan/`).
Feature parity between shell and Rust is tracked in [FEATURE-PARITY.md](FEATURE-PARITY.md).

```
tabbing-on/
├── rust/                       # Primary implementation → [layout/rust.md](layout/rust.md)
│   ├── Cargo.toml              #   Crate manifest (bin `tabbing`, v0.2.0)
│   └── src/                    #   Multi-call binary: one module per subcommand
│       └── plan/               #   tabbing-plan: voice memo → PM ticket pipeline
├── shell-impl/                 # Pure-shell implementation → [layout/shell-impl.md](layout/shell-impl.md)
│   ├── bin/                    #   CLI entry points & wrappers (17 scripts)
│   ├── lib/                    #   POSIX shared libraries (12 files)
│   └── shell/                  #   Bash/Zsh thin adapters
├── ink-plan/                   # tabbing-plan TUI prototype (Ink/React, Node >= 20)
│   ├── bin.ts                  #   CLI entry point
│   ├── src/                    #   app.tsx, components/, hooks/, services/, prompts/
│   ├── package.json            #   Dependencies (node_modules/, dist/ gitignored)
│   ├── tsconfig.json           #   TypeScript config
│   ├── Makefile                #   Build/install targets
│   └── README.md               #   Prereqs (SoX, LiteLLM/Whisper) & usage
├── docs/                       # Documentation
│   ├── PROJ-ARCH.md            #   Architecture: components, diagrams, decisions
│   ├── PROJ-ARCH.summary.md    #   Architecture quick-reference
│   ├── PROJ-LAYOUT.md          #   This file
│   ├── PROJ-LAYOUT.summary.md  #   Quick-reference tree
│   ├── FEATURE-PARITY.md       #   Shell vs Rust feature matrix
│   ├── theme-data-format.md    #   TAB_THEME_DATA 383-line blob format spec
│   ├── layout/                 #   Detailed layout breakdowns (rust.md, shell-impl.md)
│   └── assets/                 #   Documentation images (title-bar.png)
├── tmp-xdg/                    # Scratch XDG config tree for local testing (themes/)
├── .claude/                    # Claude Code agents & commands (gitignored)
├── .envrc                      # direnv — TAB_THEME + NPL_PROJECT (gitignored; run `direnv allow`)
├── .gitignore                  # Ignores .claude, .tmp, .envrc, rust/target, ink-plan artifacts
├── Makefile                    # make install: cargo build + applet symlinks + shell libs
├── CLAUDE.md                   # Claude Code project instructions
├── LICENSE                     # MIT (Copyright 2026 Keith Brings)
├── README.md                   # Project entry point
├── TODO.md                     # Roadmap & known limitations
├── plan-a.md                   # Design plan: feature extensions 1-10
├── plan-b.md                   # Design plan: _tabbing_out output conversion
├── plan-c.md                   # Design note: claude pipe listener
└── script.md                   # Demo recording script
```

## Installation (root Makefile)

`make install` compiles the Rust binary and installs it as `tabbing` with
argv0-dispatch symlinks, alongside shell-impl support files:

| Source | Installed |
|--------|-----------|
| `rust/` build → `tabbing` binary | `~/.local/bin/tabbing` |
| Applet symlinks (`tabbing-on`, `tabbing-status`, `tabbing-todo`, `tabbing-theme`, `tabbing-plan`, `task-memo`, ...) | `~/.local/bin/` → `tabbing` |
| `shell-impl/bin/_tabbing-commit` | `~/.local/bin/` (real script, not symlink) |
| `shell-impl/lib/*.sh` | `~/.local/share/tabbing-on/lib/` |
| `shell-impl/shell/tabbing.{bash,zsh}` | `~/.local/share/tabbing-on/shell/` |
| `shell-impl/direnv/tabbing.sh` | `~/.config/direnv/lib/` (`use_tabbing` helper) |

Activate in shell rc: `eval "$(tabbing-init bash|zsh)"`.

## Data Storage

Runtime state is XDG-compliant under `~/.local/state/tabbing/`
(`history/`, `todos/`, `recordings/`, `sessions/`); user themes live in
`~/.config/tabbing-on/themes/`. See [PROJ-ARCH.md](PROJ-ARCH.md) for the state
model and `TAB_*` environment variables.

## Key Files Requiring Setup

| File | Action |
|------|--------|
| `.envrc` | Run `direnv allow` (gitignored; sets TAB_THEME, NPL_PROJECT) |
| `ink-plan/` | `npm install`; needs SoX + LiteLLM endpoint for tabbing-plan TUI |
