# PROJ-FAQ — tabbing-on

Why/when/compared-to-what questions. For *what it is*, see
[PROJ-ARCH.md](PROJ-ARCH.md); for *how to do it*, see [PROJ-HOWTO.md](PROJ-HOWTO.md).

## Motivation

### Why would I use this instead of my terminal's built-in tab naming?

Because most terminals only let you set a static title, while tabbing-on gives
you a status, urgency color, emoji, scrolling marquee, and full-palette theme
that update live as your work changes state — plus a history log and per-tab
todos your terminal doesn't track at all. The honest trade-off: it's one more
thing sourced into your shell rc, and on terminals with weak OSC support
(plain `xterm`, some Windows terminals) you fall back to title-only with no
color/theme.

→ *See [howto/first-hour.md](howto/first-hour.md) to set it up.*

### Why would I want dc mode instead of the default env-var mode?

Because env mode's `TAB_*` variables are scoped to one shell process and can't
be seen or updated by background daemons, other panes, or the marquee
renderer — dc mode moves state to a shared key-value store so a daemon can
own rendering (needed for auto-marquee) and cross-process state sharing works.
The trade-off is a hard dependency on the sibling `direnv-config` utility and
a documented sharp edge: dc state is per-directory, so `cd`-ing into a
directory with prior state restores *that directory's* last title/status, not
your current tab's.

→ *See [howto/dc-mode-and-direnv.md](howto/dc-mode-and-direnv.md).*

### Why keep a pure-shell implementation around instead of shipping only the Rust binary?

Because compiled binaries cannot mutate their parent shell's environment —
the in-shell adapters, `_tabbing-commit`, and the dc-mode daemon must run
*in-process* to export `TAB_*` vars and hook `precmd`/`PROMPT_COMMAND`, which
only shell code can do. The Rust binary owns everything else (CLI dispatch,
the interactive theme picker, HTTP integrations); the two share state files
and escape sequences so they interoperate on one machine.

→ *See [PROJ-ARCH.md § Dual Implementation & Parity](PROJ-ARCH.md#dual-implementation--parity).*

## Fit

### When is tabbing-on the wrong tool?

When you're scripting output for a non-interactive/CI context, or working over
plain `xterm`/most Windows terminals with no OSC 4/10/11/12 or 1337 support —
you'll get a bare title and lose color, theme, and badge features. It's also
not a fit if you want state to survive independent of a running shell process
in env mode; that requires opting into dc mode with its own daemon and
directory-scoped caveat.

→ *See [howto/ssh-term-override.md](howto/ssh-term-override.md) and the
[Terminal Support matrix](../README.md#terminal-support) in the README.*

### Can I use tabbing-on from a non-interactive subshell or CI script?

Partially. State-setting commands (`tabbing-on`, `tabbing-status`, `--done
<id>`) work fine as standalone wrappers, but anything that reads from a TTY —
`tabbing-todo --switch`/`--pick` interactive selection, `--record` (spawns an
asciinema sub-shell) — will not work headless. Pass the target explicitly
(e.g. `tabbing-todo --done <id>`) instead of relying on the interactive
picker.

→ *See [TODO.md § Known Limitations](../TODO.md).*

## Comparison

### How does `tabbing-style` differ from `tabbing-on`?

`tabbing-on` sets title *and* status together and is the primary entry point;
`tabbing-style` is a shell-only convenience wrapper for appearance-only
changes (color/theme/bg) when you don't want to touch status text. It has no
Rust equivalent — it's handled by the shell adapter, same as `tabbing-off` and
`tabbing-daemon`.

→ *See [docs/FEATURE-PARITY.md](FEATURE-PARITY.md).*

### How does `tabbing-plan` differ from the `ink-plan/` prototype still in the repo?

`tabbing-plan`/`task-memo` is the Rust-native successor and the one you should
use; `ink-plan/` (a Node/Ink TUI) is an earlier prototype of the same
record→transcribe→classify→write flow, kept around for reference and for
anyone actively working on the prototype itself, not as an alternative
day-to-day tool. There's no feature reason to prefer it — it predates the
Rust implementation and isn't being extended.

→ *See [howto/voice-memo-tickets.md](howto/voice-memo-tickets.md).*

### How does the Rust theme picker differ from the shell one?

Both apply the same `.themedata` files and hit the same 256-color store, but
the Rust picker (ratatui) adds category-grouped headers, 60+ searchable theme
descriptions and tags, Tab/Shift-Tab category jumping, and an inline color
editor with live code preview — none of which exist in the shell picker's TTY
read loop. If you install shell-only (`make install-shell`), you get the
plainer picker.

→ *See [howto/theme-picker.md](howto/theme-picker.md) and
[FEATURE-PARITY.md § Interactive Theme Picker](FEATURE-PARITY.md).*

## Capability

### Can it actually change my terminal's whole color scheme, not just the tab title?

Yes — `--theme NAME` recolors background, foreground, cursor, and the full
16/256-color palette via OSC 4/10/11/12, gated on what the detected terminal
supports; `--bg` alone sets just the background. This surprises people who
expect only an OSC-0 title change. The catch: theming requires a terminal
that implements those OSC codes (iTerm2, Ghostty, Kitty, WezTerm, Alacritty
do; plain xterm/most Windows terminals mostly don't), and Ghostty/Kitty may
clobber it back with their own shell-integration titling unless `tabbing-doctor`
has patched their config.

→ *See [howto/theme-picker.md](howto/theme-picker.md) and
[howto/fix-terminal-titles.md](howto/fix-terminal-titles.md).*

### Can I turn a spoken thought into a project ticket without typing it?

Yes — `tabbing-plan`/`task-memo` captures a voice memo, transcribes it via
Whisper, classifies it with an LLM, and writes a structured user-story/bug/task
file. This is the one feature with an external dependency beyond the terminal
itself (SoX for mic capture, a Whisper/LLM endpoint), so it's the most likely
thing to be broken by environment drift.

→ *See [howto/voice-memo-tickets.md](howto/voice-memo-tickets.md).*

### Can `tabbing-theme` save the theme I'm using back into my project's `.envrc` for me?

No — `tabbing-theme save` exists as a subcommand but prints "not yet wired
from CLI"; it's a stub, not a bug. Set `TAB_THEME=<name>` in `.envrc` by hand
until it lands.

→ *See [howto/theme-picker.md](howto/theme-picker.md) and
[howto/dc-mode-and-direnv.md](howto/dc-mode-and-direnv.md).*

### Why does switching my active todo also change my tab's status/emoji/urgency?

Because the todo you're focused on *is* your current status by definition —
`tabbing-todo --pick`/`--switch` writes the picked todo's text/emoji/priority
into `TAB_STATUS`/`TAB_EMOJI`/`TAB_URGENCY` so the tab always reflects what
you're actually doing, instead of you maintaining two things in sync by hand.
The trade-off: switching todos overwrites any status you set manually via
`tabbing-status` since the last switch, so a hand-typed status note can get
clobbered without warning.

→ *See [howto/daily-tab-workflow.md](howto/daily-tab-workflow.md).*

## Caveats

### Does dc mode introduce a security or multi-user concern?

Not a security boundary concern — it's a local file store, not networked —
but it is a shared-state concern: the dc store is protected by an exclusive
flock for read-modify-write ops (set/yaml/unset/prune/purge/bump), so
concurrent `tabbing-on` invocations won't lose writes, but unparseable layers
now error loudly instead of being silently overwritten (a change from earlier
milestones). Env mode has no such store at all — state dies with the shell
process, which is safer by default but doesn't survive subshells.

→ *See [PROJ-ARCH.md § DC Mode](PROJ-ARCH.md#dc-mode-tabbing_on_dc_mode1).*

### What happens to old history/todo/recording files — do they clean up automatically?

No. `history/`, `todos/`, and `recordings/` under `~/.local/state/tabbing/`
accumulate indefinitely per `TAB_ID`; there is no automatic age-based sweep
yet (tracked as a known gap). Use `tabbing-clear --all`/`--everything`
manually, or clean the XDG state dir directly if it grows large.

→ *See [TODO.md § Known Limitations](../TODO.md).*

### Will tabbing-on break my SSH sessions to other machines?

It can, if you don't handle `TERM` — exporting `xterm-ghostty`/`xterm-kitty`
locally and then SSHing to a host whose `terminfo` database lacks that entry
breaks remote line editing and colors. tabbing-on doesn't cause this by
itself, but by encouraging advanced terminal features it makes you more likely
to hit it than someone on a plain `xterm`.

→ *See [howto/ssh-term-override.md](howto/ssh-term-override.md).*

## Trust

### Does any of my tab title/status/todo data leave my machine?

No, except for the two integrations you opt into explicitly: Toggl Track (if
you enable time-entry mirroring) and the Whisper/LLM endpoint used by
`tabbing-plan`/`task-memo` for voice-memo transcription and classification.
Everything else — history, todos, recordings, theme data, session state — is
local files under `~/.local/state/tabbing/` and `~/.config/tabbing-on/`, or
(in dc mode) the local `direnv-config` store.

→ *See [PROJ-ARCH.md § Persistent State](PROJ-ARCH.md#persistent-state) and
[howto/time-tracking-reports.md](howto/time-tracking-reports.md).*
