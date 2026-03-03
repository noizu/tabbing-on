# Project Layout — Summary

```
tabbing-on/
├── bin/                            # Entry points
│   ├── tabbing-init                #   Shell bootstrapper
│   └── demo-runner                #   Demo runner
├── lib/                            # POSIX shared libraries
│   ├── core.sh                     #   Colors, emoji, rendering
│   ├── history.sh                  #   Tab history tracking
│   ├── recording.sh               #   asciinema integration
│   ├── terminal.sh                 #   Terminal detection
│   └── todo.sh                     #   Todo management
├── shell/                          # Shell integrations
│   ├── tabbing.bash                #   Bash commands
│   └── tabbing.zsh                 #   Zsh commands
├── demo/                           # Demo scripts
│   └── showcase.demo
├── tests/                          # Tests (empty)
├── docs/                           # Documentation
├── LICENSE                         # MIT
├── script.md                       # Demo reference
└── terminal-utils.zshrc            # Legacy shim
```
