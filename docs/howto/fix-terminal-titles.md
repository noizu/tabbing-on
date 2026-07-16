# How to: fix a terminal that won't keep the tab title tabbing-on sets

**Goal:** Stop Ghostty/Kitty from clobbering your tab title/theme with their
own shell-integration title logic.
**Prereqs:** none — this is the first thing to run if titles "flicker back" or never change.

1. Run the diagnostic:
   ```bash
   tabbing-doctor
   ```
   It detects your terminal, checks its config for title-overriding settings,
   and reports `OK`/`!!`/`..` per check (Ghostty, Kitty, and SSH `TERM` handling).

2. Where it reports a problem, it prints the exact fix — typically a config
   line to add. Ghostty/Kitty need a directive disabling their own
   shell-integration title updates so tabbing-on's OSC 0 writes stick.

3. Re-run `tabbing-doctor` after applying its suggested fix to confirm it's
   now `OK`, then open a new terminal tab to pick up the config change.

**Verify:** set a title with `tabbing-on "test"` and switch away and back to
the tab — the title should stay, not revert.

**Gotchas:**
- **Doctor says it's fine but titles still don't stick:** you likely need a
  *new* terminal window/tab, not just a new shell — terminal-level config
  changes apply at terminal-process startup.
- **Also flagged an SSH `TERM` issue:** that's a separate, unrelated check —
  see [howto/ssh-term-override.md](ssh-term-override.md).
- **Terminal isn't Ghostty or Kitty:** most other supported terminals (iTerm2,
  WezTerm, Alacritty, Terminal.app) work out of the box; `tabbing-doctor` will
  say so and there's nothing to patch.
