# How to: see what you worked on and how long you spent

**Goal:** Turn the automatic history log tabbing-on already keeps into a
time-in-state report, and optionally push the same status changes to Toggl Track.
**Prereqs:** Some history — every `tabbing-on`/`tabbing-status` call is logged automatically, nothing to opt into.

1. See time distribution for the current tab:
   ```bash
   tabbing-report                # ASCII bar chart
   tabbing-report --mermaid      # Mermaid pie chart syntax (paste into a doc)
   ```

2. Across all tabs, or search history by text:
   ```bash
   tabbing-report --all
   tabbing-report --list             # list all known tab IDs
   tabbing-report --search "deploy"  # search this tab's history
   tabbing-history                   # list all known tabs
   tabbing-history "deploy"          # search across ALL tabs' history
   ```

3. (Optional) Mirror status changes into Toggl Track automatically — set once
   in your shell rc or `.envrc`:
   ```bash
   export TAB_TOGGL_TOKEN="<your Toggl API token>"
   export TAB_TOGGL_WORKSPACE="<workspace id>"   # optional
   export TAB_TOGGL_PROJECT="<project id>"       # optional
   ```
   With `TAB_TOGGL_TOKEN` set, every `tabbing-on`/`tabbing-status` title or
   status change starts/updates a Toggl time entry; `tabbing-off` stops it.

**Verify:**
```bash
tabbing-report --list      # confirm your tab IDs are present
tabbing-info               # `history:` path shows where the YAML log lives
```

**Gotchas:**
- **No Toggl integration happening:** every Toggl call is a no-op if
  `TAB_TOGGL_TOKEN` is unset — this is by design, not a bug. Check `echo
  $TAB_TOGGL_TOKEN`.
- **Report shows 0% for everything:** the report buckets by *status text*
  changes over time — if you only ever called `tabbing-on` once and never
  updated status, there's only one bucket to show.
- **History grows forever:** prune it with `tabbing-clear history --before
  2026-01` (single tab) or `tabbing-clear history --all`.
