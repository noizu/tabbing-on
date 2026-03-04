# TODO

## CLI Wrappers for Subshell Compatibility

### Problem

The 7 user-facing commands (`tabbing-on`, `tabbing-status`, `tabbing-todo`,
`tabbing-report`, `tabbing-history`, `tabbing-recordings`, `tabbing-info`) are
**shell functions**, not executables. They only exist in the shell session that
ran `eval "$(tabbing-init bash)"`.

Tools like Claude Code spawn fresh subshells for each command — those subshells
don't inherit the parent's functions. Environment variables *do* pass through,
but function definitions do not.

### Core Challenge: State Propagation

Even if we create `bin/` wrapper scripts that source the libraries, each
invocation runs in its own subshell. Env var changes (`TAB_TITLE`, `TAB_STATUS`,
etc.) made inside a wrapper die with that subshell — the parent shell and
subsequent wrapper calls never see them.

This is a **message-passing problem**: the wrapper subshells need a way to
communicate state changes back to the parent shell (or at least to each other).

### Potential Approaches

#### 1. State File (simplest)

Wrapper scripts read/write a shared state file (`$STATE_DIR/env`) on each call.

```
# ~/.local/state/tabbing/env
TAB_ID=a1b2c3d4
TAB_TITLE=MyProject
TAB_STATUS=building
TAB_URGENCY=2
TAB_EMOJI=rocket
TAB_HIGHLIGHT=blue
TAB_TERMINAL=iterm2
```

- **Load** at start of each wrapper (eval the file)
- **Save** after the function runs (write current env vars back)
- Terminal escapes (OSC title set, tab color) still work — they go straight to
  the terminal regardless of subshell depth
- History recording (file I/O) works fine too

**Pros**: Simple, no daemon, works immediately.
**Cons**: Race conditions if multiple wrappers run concurrently. Parent shell's
env stays stale unless it also polls the file (via prompt hook).

#### 2. Named Pipe / FIFO

Parent shell runs a background listener on a named pipe. Wrappers write
state-change messages to the pipe. Parent reads and `export`s accordingly.

```sh
# Parent side (in prompt hook or background job)
while read -r line < "$STATE_DIR/fifo"; do
  eval "$line"  # e.g. export TAB_TITLE="MyProject"
done

# Wrapper side
echo 'export TAB_TITLE="MyProject"' > "$STATE_DIR/fifo"
```

**Pros**: Real-time propagation to parent. No polling.
**Cons**: More complex setup. Needs background process in parent. Platform
differences (mkfifo availability). Cleanup on shell exit.

#### 3. Prompt-Hook Sync

Parent shell's prompt hook (`precmd`/`PROMPT_COMMAND`) reads the state file
before each prompt. Wrappers write to the state file. Parent picks up changes
on next prompt.

This is a hybrid of (1) and (2) — uses a file but the parent actively syncs.

**Pros**: No daemon, no FIFO. Leverages existing prompt hook infrastructure.
**Cons**: Changes only visible after next prompt (slight delay). Still has the
stale-env problem *during* a pipeline of commands.

#### 4. Socket-Based IPC

A lightweight daemon (or coprocess) mediates state. Wrappers send updates via
Unix domain socket. Parent queries on demand.

**Pros**: Clean IPC, handles concurrency.
**Cons**: Way overengineered for this use case. Needs a daemon lifecycle.

### Recommended Path

**Start with (1) + (3)**: State file for persistence, prompt-hook sync for
parent awareness. This covers the Claude Code use case (wrappers share state
via file) and the interactive use case (parent picks up changes at each prompt).

#### Implementation Sketch

1. **`lib/cli-state.sh`** — POSIX functions:
   - `_tabbing_state_save()` — write env vars to `$STATE_DIR/env`
   - `_tabbing_state_load()` — eval `$STATE_DIR/env` if it exists
   - `_tabbing_state_clear()` — remove state file

2. **`bin/tabbing-on`** (and friends) — thin bash wrappers:
   ```bash
   #!/usr/bin/env bash
   TABBING_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
   . "$TABBING_ROOT/lib/cli-state.sh"
   _tabbing_state_load
   . "$TABBING_ROOT/shell/tabbing.bash"
   tabbing-on "$@"
   _tabbing_state_save
   ```

3. **Prompt hook addition** — in `shell/tabbing.{bash,zsh}`:
   ```bash
   _tabbing_prompt_sync() {
     _tabbing_state_load  # pick up changes from wrappers
   }
   ```

### Edge Cases to Address

- **First run**: No state file yet — wrappers should work without one
- **Stale state**: If user closes a tab, old state lingers — need TTY or
  session-based scoping (maybe key state file by `$TAB_ID` or `$$`)
- **Multiple terminals**: Each terminal needs its own state — scope by TAB_ID
  or terminal PID
- **Interactive features**: `tabbing-todo --switch` uses `read` — may not work
  in non-interactive subshells (Claude). Could accept an ID arg as fallback
- **Recording**: `--record` spawns an asciinema sub-shell — fundamentally
  incompatible with non-interactive wrappers. Skip or document limitation
