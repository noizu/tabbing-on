# Project Layout — Summary

```
tabbing-on/
├── rust/                       # Primary Rust impl (v0.2.0) → layout/rust.md
│   ├── Cargo.toml              #   Crate manifest
│   └── src/                    #   Multi-call binary, one module per subcommand
│       └── plan/               #   tabbing-plan: voice memo → ticket
├── shell-impl/                 # Pure-shell impl → layout/shell-impl.md
│   ├── bin/                    #   CLI entry points (17 scripts)
│   ├── lib/                    #   POSIX libraries (12 files)
│   ├── shell/                  #   Bash/Zsh adapters
│   ├── direnv/                 #   use_tabbing direnv helper
│   ├── data/                   #   X11 color database
│   ├── examples/themes/        #   User theme templates
│   ├── demo/                   #   Demo scripts & recordings
│   └── terminal-utils.zshrc    #   Legacy shim
├── ink-plan/                   # tabbing-plan TUI prototype (Ink/React)
├── docs/                       # PROJ-ARCH, FEATURE-PARITY, theme-data-format, layout/, assets/
├── tmp-xdg/                    # Scratch XDG config tree for testing
├── .envrc                      # direnv (gitignored) — TAB_THEME, NPL_PROJECT
├── Makefile                    # make install: cargo build + symlinks + shell libs
├── CLAUDE.md                   # Claude Code instructions
├── LICENSE                     # MIT
├── README.md                   # Project entry point
├── TODO.md                     # Roadmap
├── plan-a.md / plan-b.md / plan-c.md  # Design plans
└── script.md                   # Demo recording script
```
