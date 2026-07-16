# PROJ-HOWTO — Summary (task list)

## First hour
- **Install tabbing-on and get a tab title working** — Go from a fresh checkout to a live, colored tab title in your shell.

## Recurring workflow
- **Run the day-to-day tab title / status / todo workflow** — Set a tab, update its status, track a per-tab todo list, and clear it when done.
- **See what you worked on and how long you spent** — Turn the automatic history log into a time-in-state report, optionally mirrored to Toggl Track.

## Non-obvious capabilities
- **Browse, apply, and customize terminal color themes** — Recolor your whole terminal — background, palette, cursor, prompt layout — via the interactive picker or by name.
- **Turn a spoken thought into a project ticket file** — Record a voice memo and get back a structured user-story/bug/task ticket file via Whisper + LLM.
- **Share tab state across processes with direnv-config (dc) mode** — Let a background daemon render your tab title from a shared store — needed for auto-marquee and cross-process state.

## Sharp edges
- **Fix a terminal that won't keep the tab title tabbing-on sets** — Stop Ghostty/Kitty from clobbering your tab title/theme with their own shell-integration logic.
- **Stop remote SSH sessions from breaking on your local TERM** — Avoid broken remote sessions caused by advanced local `TERM` values like `xterm-ghostty`/`xterm-kitty`.
