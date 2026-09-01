# How to: install tabbing-on and get a tab title working

**Goal:** Go from a fresh checkout to a live, colored tab title in your shell.
**Prereqs:** Rust/cargo for the primary build (or skip to shell-only fallback); a terminal that supports OSC 0 titles (all of them do).

1. Build and install the Rust binary + shell adapters + symlinks:
   ```bash
   cd utilities/shell/tabbing-on
   make install
   ```
   This puts `tabbing-on` (and ~18 applet symlinks) in `~/.local/bin/`, the shell
   adapters in `~/.local/share/tabbing-on/shell/`, and libs in `~/.local/share/tabbing-on/lib/`.

2. Hook it into your shell rc:
   ```bash
   # ~/.zshrc
   eval "$(tabbing-init zsh)"

   # ~/.bashrc
   eval "$(tabbing-init bash)"
   ```
   Open a new shell (or `source ~/.zshrc`) to pick it up.

3. Set a tab title and status:
   ```bash
   tabbing-on "MyApp" "deploying" -blue -rocket -pri2
   ```

**Verify:**
```bash
tabbing-on            # re-prints current title/status with no args
tabbing-info          # full state dump incl. file paths
```
Your terminal's tab bar should now show "MyApp" with a blue highlight and a
rocket emoji indicator.

**Gotchas:**
- **No `cargo` available?** Use the legacy pure-shell install instead: `make install-shell` (installs `shell-impl/bin/*` directly — same commands, no Rust needed).
- **Title doesn't change on Ghostty or Kitty:** these terminals ship config that overrides tab-set titles. Run `tabbing-doctor` — it detects and patches this (see [howto/fix-terminal-titles.md](fix-terminal-titles.md)).
- **`tabbing-status`/`tabbing-todo` say `no TAB_TITLE set`:** you must call `tabbing-on` at least once per shell before any of the other commands — they operate on the state it creates.
