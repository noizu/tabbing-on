# Layout: shell-impl/

Original pure-shell implementation. Three-layer design: POSIX libraries,
Bash/Zsh adapters, and CLI entry points (see [../PROJ-ARCH.md](../PROJ-ARCH.md)).

```
shell-impl/
├── bin/                            # Entry points & CLI wrappers
│   ├── tabbing-init                #   Shell bootstrapper — eval "$(tabbing-init bash|zsh)"
│   ├── tabbing-daemon              #   Background daemon: polls dc, renders title, marquees
│   ├── demo-runner                 #   Typewriter-style interactive demo runner
│   ├── _tabbing-wrapper            #   Shared setup: sources adapter + all libs, loads session
│   ├── _tabbing-commit             #   Side-effects helper: history, display, session save
│   ├── tabbing-on                  #   CLI: set/display tab title & status
│   ├── tabbing-status              #   CLI: update status
│   ├── tabbing-theme               #   CLI: theme browser and selector
│   ├── tabbing-marquee             #   CLI: scrolling marquee text in tab title
│   ├── tabbing-todo                #   CLI: manage todos
│   ├── tabbing-report              #   CLI: time-in-state reports (ASCII/Mermaid)
│   ├── tabbing-history             #   CLI: search/browse history
│   ├── tabbing-recordings          #   CLI: manage asciinema recordings
│   ├── tabbing-info                #   CLI: full state dump
│   ├── tabbing-clear               #   CLI: clear history/todos/recordings
│   ├── tabbing-claude-statusline   #   CLI: Claude Code IDE statusline bridge
│   └── tabbing-doctor              #   CLI: check/fix terminal config (Ghostty/Kitty conflicts)
├── lib/                            # POSIX-compatible shared libraries (_tabbing_* functions)
│   ├── render.sh                   #   Render pipeline: emoji, color, title escapes, version
│   ├── core.sh                     #   Emoji list, color list, help, YAML escape
│   ├── terminal.sh                 #   Terminal detection, badge, clear
│   ├── history.sh                  #   Tab ID generation, YAML history tracking
│   ├── recording.sh                #   asciinema recording lifecycle
│   ├── session.sh                  #   Per-session state persistence (TAB_SESSION-keyed)
│   ├── todo.sh                     #   Per-tab todo management (provider pattern)
│   ├── theme.sh                    #   Theme loading, listing, custom theme files
│   ├── theme-data.sh               #   TAB_THEME_DATA 383-line blob get/set/generate
│   ├── dc.sh                       #   direnv-config integration + daemon lifecycle
│   ├── claude.sh                   #   Claude Code IDE bridge (FIFO + state files)
│   └── toggl.sh                    #   Toggl time tracking integration
├── shell/                          # Shell-specific thin adapters
│   ├── tabbing.bash                #   Bash: sources render.sh + dc.sh, public functions
│   └── tabbing.zsh                 #   Zsh: sources render.sh + dc.sh, public functions
├── direnv/
│   └── tabbing.sh                  #   direnv stdlib helper: use_tabbing "project" ...
├── data/
│   └── colors.txt                  #   X11 symbolic color database (name|#RRGGBB)
├── examples/
│   └── themes/                     #   User theme templates
│       ├── my-dark.theme           #     Full 19-key theme example
│       └── minimal.theme           #     Minimal 2-key theme (bg + fg only)
├── demo/                           # Demo scripts & recordings
│   ├── showcase.demo               #   Interactive feature walkthrough
│   ├── showcase.cast               #   asciinema recording of demo
│   └── showcase.gif                #   GIF render of demo
└── terminal-utils.zshrc            # LEGACY compatibility shim (prefer tabbing-init)
```
