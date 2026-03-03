# tabbing-on

A pure-shell terminal tab title, status & task manager.
Works with iTerm2, Ghostty, Kitty, WezTerm, Alacritty, and more.

> **[`demo-runner`](#demo-runner)** — a typewriter-style demo player is included. Run `bin/demo-runner` for a guided walkthrough.



https://github.com/user-attachments/assets/dcefea35-beaa-43c8-afe6-9717a478e7cf


## Install

```bash
# Zsh — add to .zshrc
eval "$(path/to/tabbing-on/bin/tabbing-init zsh)"

# Bash — add to .bashrc
eval "$(path/to/tabbing-on/bin/tabbing-init bash)"
```

Or source the shell file directly:

```bash
source path/to/tabbing-on/shell/tabbing.zsh   # zsh
source path/to/tabbing-on/shell/tabbing.bash   # bash
```

## Commands

### `tabbing-on`

The primary command. Sets the tab title, status, highlight color, urgency, and emoji.

```bash
tabbing-on "MyApp" "deploying"                    # title + status
tabbing-on "MyApp" -blue "deploying" -rocket -p2  # with color, emoji, urgency
tabbing-on                                         # display current state
tabbing-on emojis                                  # list available emojis
tabbing-on colors                                  # list available colors
tabbing-on help                                    # show help
```

| Flag | Aliases | Description |
|---|---|---|
| `--highlight COLOR` | `--color`, `-h COLOR`, `-COLOR` | Title highlight color (`-blue`, `-red`, etc.) |
| `--urgency N` | `--pri N`, `-p N`, `-priN` | Urgency 0-5 (0 = critical/red, 5 = nominal/gray) |
| `--emoji NAME` | `-e NAME`, `-NAME` | Named emoji indicator (`-rocket`, `-fire`, etc.) |
| `--no-emoji` | | Clear emoji |
| `--record` | | Start an asciinema recording |
| `--continue` | | Keep current recording across status changes |
| `--stop-recording` | | Stop active recording |
| `--terminal-info` | | Show detected terminal and feature support |

### `tabbing-status`

Update just the status portion of the tab title. Requires `tabbing-on` to have been called first.

```bash
tabbing-status "building..."
tabbing-status -fire "hotfix in progress"
tabbing-status -pri0 "DEADLINE TOMORROW"
```

Accepts the same urgency, emoji, and recording flags as `tabbing-on`.

### `tabbing-todo`

Per-tab todo/task management.

```bash
tabbing-todo "K8 infra setup" -e gear -m "Deploy to staging cluster"  # add
tabbing-todo                                                           # list
tabbing-todo --pick                                                    # switch active todo
tabbing-todo --done                                                    # mark active as done
tabbing-todo --done 3                                                  # mark #3 as done
```

| Flag | Aliases | Description |
|---|---|---|
| `-m "desc"` | `--message` | Description text |
| `--emoji NAME` | `-e NAME`, `-NAME` | Attach emoji |
| `--urgency N` | `--pri N`, `-p N` | Set urgency |
| `--pick` | `-n`, `--switch` | Interactive: pick a todo to work on |
| `--done [id]` | | Mark done (defaults to active) |

Switching todos updates `TAB_STATUS`, `TAB_EMOJI`, and `TAB_URGENCY` automatically.

### `tabbing-report`

Time-in-state reports computed from event history.

```bash
tabbing-report                   # ASCII bar chart for current tab
tabbing-report --mermaid         # Mermaid pie chart syntax
tabbing-report --all             # all tabs
tabbing-report --list            # list all known tabs
tabbing-report --search "query"  # search history
```

### `tabbing-history`

Browse and search across all tab history.

```bash
tabbing-history              # list all known tabs
tabbing-history "deploying"  # search all history files
```

### `tabbing-recordings`

List and manage asciinema `.cast` recordings.

```bash
tabbing-recordings                              # list for current tab
tabbing-recordings --tab TAB_ID                 # list for specific tab
tabbing-recordings --to-gif recording.cast      # convert to GIF (requires agg)
```

### `tabbing-info`

Full state dump for a tab: env vars, file paths, history count, todos, recordings.

```bash
tabbing-info              # current tab
tabbing-info TAB_ID       # specific tab
```

### `tabbing-clear`

Delete stored data.

```bash
tabbing-clear history                    # clear history (current tab)
tabbing-clear history --all              # clear history (all tabs)
tabbing-clear history --before 2026-01   # clear entries before date
tabbing-clear todos                      # clear todos (current tab)
tabbing-clear recordings                 # clear recordings (current tab)
tabbing-clear all                        # clear everything (current tab)
tabbing-clear everything                 # nuke all data for ALL tabs
```

## Environment Variables

| Variable | Description |
|---|---|
| `TAB_TITLE` | Tab title text |
| `TAB_STATUS` | Status sub-text |
| `TAB_HIGHLIGHT` | Color name for title highlight |
| `TAB_URGENCY` | 0-5 (0 = critical/red, 5 = nominal/gray) |
| `TAB_EMOJI` | Named emoji (overrides the urgency dot) |
| `TAB_ID` | 8-char hex ID, unique per tab (auto-generated) |
| `TAB_TERMINAL` | Detected terminal emulator |
| `TAB_RECORDING` | Path to active `.cast` file |

## Data Storage

All state lives under `~/.local/state/tabbing/` (or `$XDG_STATE_HOME/tabbing/`):

```
tabbing/
  history/{TAB_ID}.yaml           # timestamped event log
  todos/{TAB_ID}.yaml             # todo items
  recordings/{TAB_ID}/*.cast      # asciinema recordings
```

## License

MIT - Copyright 2026 Keith Brings

---

## demo-runner

A typewriter-style script player for terminal demos. It reads `.demo` files and plays them back with typed-out commands, colored headings, and real command execution.

```bash
bin/demo-runner                        # run the default showcase
bin/demo-runner my-script.demo         # run a custom script
bin/demo-runner --fast                 # speed up typing
bin/demo-runner --slow                 # slow down typing
bin/demo-runner --speed medium         # fast | medium | slow
bin/demo-runner --no-pause             # minimal pauses between commands
```

The runner sources `tabbing.zsh` so all `tabbing-*` commands work inside demo scripts.

### `.demo` File Format

| Line prefix | Behavior |
|---|---|
| `# text` | Bold cyan heading with underline (appears instantly) |
| `## text` | Dim description (appears instantly) |
| `$ command` | Typed out char-by-char, then executed via `eval` |
| `@sleep N` | Pause for N seconds |
| `@input TEXT` | Simulate typed stdin input |
| `@prompt TEXT` | Pause and wait for Enter (shows TEXT) |
| `@clear` | Clear the screen |
| *(blank line)* | Outputs a blank line |
| *(anything else)* | Printed as-is |

Example `.demo` file:

```
# My Feature

## This shows off the new widget.

$ echo "Hello, world!"

@sleep 1

$ ls -la
```

Speed settings control the character delay: fast (0.01s), medium (0.03s), slow (0.06s).
The `TABBING_DEMO_SPEED` environment variable can also set the default speed.
