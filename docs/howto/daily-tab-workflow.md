# How to: run the day-to-day tab title / status / todo workflow

**Goal:** The 4 commands you'll actually type most days — set a tab, update its
status as work progresses, track a per-tab todo list, and clear it when done.
**Prereqs:** [First-hour setup](first-hour.md) done — `tabbing-on` called at least once this shell.

1. Start the tab for whatever you're working on:
   ```bash
   tabbing-on "backend" "reviewing PR #42" -blue -eyes
   ```

2. Update just the status as things change (title/color/emoji persist):
   ```bash
   tabbing-status "running tests"
   tabbing-status -fire "hotfix — prod down" -pri0   # bump urgency for incidents
   ```

3. Track a todo for the tab, and switch focus between several:
   ```bash
   tabbing-todo "fix flaky test" -e gear -m "CI job #881 intermittent"
   tabbing-todo                 # list todos for this tab
   tabbing-todo --pick          # interactively switch active todo
   tabbing-todo --done          # mark the active one done
   tabbing-todo --done 3        # or mark #3 done directly (non-interactive/Claude-safe)
   ```
   Switching todos auto-updates `TAB_STATUS`, `TAB_EMOJI`, and `TAB_URGENCY` to match.

4. Wind down / hand off the tab:
   ```bash
   tabbing-off                  # clear title, color, theme, badge — restore terminal defaults
   ```

**Verify:**
```bash
tabbing-info          # confirm title/status/emoji/urgency match what you set
tabbing-todo          # confirm todo list state
```

**Gotchas:**
- **`tabbing-todo --pick` hangs or does nothing:** it reads from `/dev/tty` for
  interactive selection — it won't work from a non-interactive subshell (e.g. a
  Claude Code tool call). Use `tabbing-todo --done <id>` or pass the id directly instead.
- **Status flags reset the emoji/color unexpectedly:** `tabbing-status` accepts
  the same `-color`/`-emoji`/`-pri` flags as `tabbing-on` — pass only what you
  want to change; omitted flags keep their current value.
- **Want a full week-in-review, not just current state?** See
  [howto/time-tracking-reports.md](time-tracking-reports.md).
