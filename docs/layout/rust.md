# Layout: rust/

Primary Rust implementation (v0.2.0). Single multi-call binary `tabbing` that
dispatches on argv0 — installed applet symlinks (`tabbing-on`, `tabbing-status`,
`tabbing-theme`, `tabbing-plan`, `task-memo`, ...) select the subcommand.
Feature coverage vs shell-impl: [../FEATURE-PARITY.md](../FEATURE-PARITY.md).

```
rust/
├── Cargo.toml                  # Crate manifest (clap, reqwest, direnv_config, ...)
├── Cargo.lock                  # Locked dependency versions
├── target/                     # Build output (gitignored)
└── src/
    ├── main.rs                 # argv0 dispatch, VERSION const, subcommand routing
    ├── render.rs               # Render pipeline: title/status/emoji/color output
    ├── state.rs                # TAB_* state model + direnv-config (DcClient) sync
    ├── terminal.rs             # Terminal detection & escape sequences
    ├── color.rs                # Color parsing / X11 name lookup
    ├── emoji.rs                # Named emoji table & lookup
    ├── theme.rs                # Theme loading/selection
    ├── theme_data.rs           # TAB_THEME_DATA 383-line blob (mirrors theme-data.sh)
    ├── theme_picker.rs         # Interactive theme browser UI
    ├── history.rs              # Tab history tracking
    ├── todo.rs                 # Per-tab todo management
    ├── recording.rs            # asciinema recording lifecycle
    ├── marquee.rs              # Scrolling marquee mode
    ├── daemon.rs               # Background daemon (dc mode)
    ├── doctor.rs               # Terminal config checks/fixes
    ├── init.rs                 # Shell bootstrap output (tabbing-init)
    ├── claude.rs               # Claude Code IDE statusline bridge
    ├── bridge.rs               # External process/pipe bridging
    ├── toggl.rs                # Toggl time tracking integration
    ├── demo.rs                 # Demo runner
    └── plan/                   # tabbing-plan / task-memo: voice memo → PM ticket
        ├── mod.rs              #   Module exports
        ├── app.rs              #   TUI application loop
        ├── audio.rs            #   Microphone recording
        ├── llm.rs              #   Whisper transcription + LLM classification
        ├── prompt.rs           #   LLM prompt templates
        ├── config.rs           #   Endpoint/config handling
        ├── filesystem.rs       #   Ticket file output
        └── types.rs            #   Shared types
```
