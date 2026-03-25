# Project Layout — Summary

```
tabbing-on/
├── bin/                            # Entry points & CLI wrappers
│   ├── tabbing-init                #   Shell bootstrapper
│   ├── demo-runner                #   Demo runner
│   ├── _tabbing-wrapper            #   Shared setup (sources adapter + all libs)
│   ├── _tabbing-commit             #   Side-effects helper for thin adapter
│   ├── tabbing-on                  #   CLI: set title & status
│   ├── tabbing-status              #   CLI: update status
│   ├── tabbing-marquee             #   CLI: scrolling marquee text
│   ├── tabbing-todo                #   CLI: manage todos
│   ├── tabbing-report              #   CLI: reports
│   ├── tabbing-history             #   CLI: history
│   ├── tabbing-recordings          #   CLI: recordings
│   ├── tabbing-info                #   CLI: state dump
│   ├── tabbing-clear               #   CLI: clear data
│   └── tabbing-doctor              #   CLI: check/fix terminal config
├── lib/                            # POSIX shared libraries
│   ├── render.sh                   #   Render pipeline (sourced by adapter)
│   ├── core.sh                     #   Emoji/color lists, help, YAML escape
│   ├── terminal.sh                 #   Terminal detection, badge, clear
│   ├── history.sh                  #   Tab history tracking
│   ├── recording.sh               #   asciinema integration
│   ├── session.sh                  #   Session state persistence
│   └── todo.sh                     #   Todo management
├── shell/                          # Shell thin adapters
│   ├── tabbing.bash                #   Bash adapter (sources render.sh only)
│   └── tabbing.zsh                 #   Zsh adapter (sources render.sh only)
├── examples/                       # Example config files
│   └── themes/                     #   User theme templates (.theme files)
├── demo/                           # Demo scripts & outputs
│   ├── showcase.demo               #   Interactive walkthrough
│   ├── showcase.cast               #   asciinema recording
│   └── showcase.gif                #   GIF render
├── tests/                          # Tests (empty)
├── docs/                           # Documentation
│   ├── PROJ-ARCH.md                #   Architecture
│   ├── PROJ-ARCH.summary.md       #   Architecture summary
│   ├── PROJ-LAYOUT.md              #   Layout (this doc's source)
│   ├── PROJ-LAYOUT.summary.md     #   Layout summary
│   └── assets/                     #   Images
├── .envrc                          # direnv
├── .gitignore                      # Git ignore rules
├── CLAUDE.md                       # Claude Code instructions
├── LICENSE                         # MIT
├── README.md                       # Project entry point
├── TODO.md                         # Roadmap
├── script.md                       # Demo reference
└── terminal-utils.zshrc            # Legacy shim
```
