# How to: share tab state across processes with direnv-config (dc) mode

**Goal:** Let a background daemon render your tab title from a shared
key/value store instead of shell env vars — needed for automatic marquee
scrolling and state shared across subshells/processes.
**Prereqs:** [direnv-config (`dc`)](https://github.com/the-robot-lives/direnv-config) installed and on `PATH`.

1. Enable dc mode in your shell rc:
   ```bash
   # .zshrc
   export TABBING_ON_DC_MODE=1
   eval "$(tabbing-init zsh --direnv-config-mode)"
   ```

2. Use `tabbing-on`/`tabbing-status`/etc. exactly as normal — writes now go
   through the shared `dc` store (namespaced by `DC_TAB_NS`), a background
   `tabbing-daemon` polls for changes every 200ms and re-renders, and it
   auto-marquees any status text over 20 characters.

3. Set a per-project theme automatically via `.envrc`:
   ```bash
   # .envrc in a project directory
   export TAB_THEME=dracula
   ```
   With direnv wired to source this, entering the directory applies the theme.

**Verify:**
```bash
tabbing-info               # confirms current state, same as env mode
ps aux | grep tabbing-daemon   # confirms the daemon is running
```

**Gotchas:**
- **Directory theming feels "sticky" in a surprising way:** dc state is
  **per-directory, not per-tab** — entering a directory that has prior dc
  state (from a past session) restores *that* directory's last title/status/
  theme, even in a brand-new tab. This is by design. Use `tabbing-off` to
  clear it, or set new values explicitly with `tabbing-on`.
- **A fresh shell inherits stale title/theme from a previous session:** the
  adapter purges `TAB_*` on init specifically to prevent this — if you still
  see it, confirm you're re-sourcing via `tabbing-init`, not manually
  `source`-ing the adapter without the purge step.
- **Daemon not picking up changes:** it's SIGUSR1-driven for instant refresh
  plus a 200ms poll fallback — if neither happens, confirm
  `TABBING_DC_DAEMON_PID` is set and the process is alive (`ps -p
  $TABBING_DC_DAEMON_PID`).
