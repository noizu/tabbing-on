# PROJ-HOWTO — tabbing-on

Task-oriented guides for the things you'll actually want to do with
`tabbing-on`. For *what it is*, see [PROJ-ARCH.md](PROJ-ARCH.md); for *where
things live*, see [PROJ-LAYOUT.md](PROJ-LAYOUT.md).

## First hour

## How to: install tabbing-on and get a tab title working
Go from a fresh checkout to a live, colored tab title in your shell.
→ *See [howto/first-hour.md](howto/first-hour.md)*

## Recurring workflow

## How to: run the day-to-day tab title / status / todo workflow
Set a tab, update its status, track a per-tab todo list, and clear it when done.
→ *See [howto/daily-tab-workflow.md](howto/daily-tab-workflow.md)*

## How to: see what you worked on and how long you spent
Turn the automatic history log into a time-in-state report, optionally mirrored to Toggl Track.
→ *See [howto/time-tracking-reports.md](howto/time-tracking-reports.md)*

## Non-obvious capabilities

## How to: browse, apply, and customize terminal color themes
Recolor your whole terminal — background, palette, cursor, prompt layout — via the interactive picker or by name.
→ *See [howto/theme-picker.md](howto/theme-picker.md)*

## How to: turn a spoken thought into a project ticket file
Record a voice memo and get back a structured user-story/bug/task ticket file via Whisper + LLM.
→ *See [howto/voice-memo-tickets.md](howto/voice-memo-tickets.md)*

## How to: share tab state across processes with direnv-config (dc) mode
Let a background daemon render your tab title from a shared store — needed for auto-marquee and cross-process state.
→ *See [howto/dc-mode-and-direnv.md](howto/dc-mode-and-direnv.md)*

## Sharp edges

## How to: fix a terminal that won't keep the tab title tabbing-on sets
Stop Ghostty/Kitty from clobbering your tab title/theme with their own shell-integration logic.
→ *See [howto/fix-terminal-titles.md](howto/fix-terminal-titles.md)*

## How to: stop remote SSH sessions from breaking on your local TERM
Avoid broken remote sessions caused by advanced local `TERM` values like `xterm-ghostty`/`xterm-kitty`.
→ *See [howto/ssh-term-override.md](howto/ssh-term-override.md)*

## Everything else

The full command reference (flags, aliases, env vars, theme file format,
terminal support matrix) lives in the project [README.md](../README.md) —
this doc covers the tasks the README's reference tables don't make obvious.
