# Project Architecture — Summary

## Overview

Terminal tab title/status/todo/theme/recording manager with two parallel implementations: primary Rust multi-call binary (rust/, v0.2.0, argv0 applet dispatch) and the original pure-shell tree (shell-impl/, POSIX libs + Bash/Zsh adapters), plus a retained Ink/React prototype of tabbing-plan (ink-plan/). Integrates with Claude Code IDE statusline, Toggl Track, and Whisper/LLM (voice memo → PM ticket). Two runtime modes: env mode (TAB_* shell variables) and dc mode (direnv-config store + background daemon rendering titles).

## Components

- **rust/src/main.rs** — argv0 multi-call dispatch to subcommand modules
- **rust/src/** modules — render, state (dc write-through), terminal, theme + ratatui theme picker, history, todo, recording, doctor, claude/bridge, toggl, marquee, init, demo
- **rust/src/plan/** — tabbing-plan / task-memo: mic capture → Whisper transcription → LLM classification → ticket files
- **shell-impl/lib/*.sh** — POSIX libraries (_tabbing_* prefix): render, core, terminal, history, session, todo, theme, theme-data, dc, claude, toggl, recording
- **shell-impl/shell/tabbing.{bash,zsh}** — in-shell adapters (env exports persist; precmd/PROMPT_COMMAND hooks)
- **shell-impl/bin/** — CLI wrappers, _tabbing-commit (real script, not symlink), tabbing-daemon (dc-mode poller/marquee, shell-only), demo-runner
- **ink-plan/** — Ink/React (Node >= 20) tabbing-plan prototype; needs SoX + LiteLLM/Whisper

## Dual Implementation

Rust owns the CLI surface and interactive-heavy features (ratatui theme picker, plan TUI, HTTP). Shell owns what must run in the interactive shell: adapters, _tabbing-commit, daemon, demo-runner. Both share state files, dc keys, and escape sequences. Parity matrix: FEATURE-PARITY.md.

## State

- **Env mode**: TAB_TITLE/STATUS/HIGHLIGHT/URGENCY/EMOJI/THEME + auto TAB_ID (per tab), TAB_SESSION, TAB_TERMINAL
- **DC mode** (TABBING_ON_DC_MODE=1): write-through to dc `tab` namespace; single mutation path bumps last_update (ms epoch) and SIGUSR1s the daemon; daemon polls at 200ms, marquees status > 20 chars
- **Persistent**: XDG `~/.local/state/tabbing/` (history/, todos/, recordings/, sessions/, claude bridge FIFO + state + pid); user themes in `~/.config/tabbing-on/themes/`

## Terminal Abstraction

Env-var detection (iTerm2 > Ghostty > Kitty > WezTerm > ... > xterm); OSC 0 titles universal, OSC 6 tab color + OSC 1337 badge (iTerm2), Kitty remote control, OSC 4/10/11/12 theme recoloring. tabbing-doctor patches Kitty/Ghostty configs that override titles.

## Installation & Ecosystem

Lives at utilities/shell/tabbing-on in the Noizu Infra monorepo; a SUBDIR of utilities/shell/Makefile, so repo-root `make install-utilities` recurses into its Makefile. No share/k8-lib or .infra-config.yaml footprint — purely local tool sharing the ~/.local/bin convention. `make install`: cargo build → ~/.local/bin/tabbing-on + ~19 applet symlinks + shell libs to ~/.local/share/tabbing-on/ + direnv use_tabbing helper. `make install-shell` = legacy pure-shell install. Activate via `eval "$(tabbing-init bash|zsh)"`. dc mode depends on sibling direnv-config utility.

## Key Decisions

- Multi-call binary with argv0 dispatch: one artifact, per-command UX
- Shell layer retained: compiled binary can't mutate parent shell env; adapters/commit/daemon stay shell, co-installed
- Single mutation path (_tabbing_set / state.rs): env export + dc write + last_update bump + SIGUSR1
- last_update as sole change sentinel; SIGUSR1 instant refresh, 200ms poll for marquee
- POSIX libs, bash/zsh-isms only in adapters; only render.sh + dc.sh sourced at init for fast prompts
- YAML via sed/awk, no hard external deps (dc, asciinema optional)
- Per-tab isolation (TAB_ID) and per-session scoping (TAB_SESSION)
- Claude bridge via FIFO decouples render pipeline from IDE statusline
- tabbing-plan prototyped in Ink/React before Rust port; prototype retained
