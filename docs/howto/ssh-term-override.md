# How to: stop remote SSH sessions from breaking on your local TERM

**Goal:** Avoid "unknown terminal type" / broken colors on remote hosts caused
by advanced local `TERM` values like `xterm-ghostty` or `xterm-kitty` that the
remote's terminfo database doesn't know.
**Prereqs:** `tabbing-doctor` has flagged an SSH warning (`No ssh wrapper — remote hosts will get TERM=...`), or you SSH out from Ghostty/Kitty.

1. Add the shim to your shell rc, after your `tabbing-init` line:
   ```bash
   # ~/.zshrc
   eval "$(tabbing-init zsh)"
   eval "$(tabbing-ssh-shim zsh)"

   # ~/.bashrc
   eval "$(tabbing-init bash)"
   eval "$(tabbing-ssh-shim bash)"
   ```
   This wraps `ssh` so it exports a safe `TERM_FOR_SSHD` value (e.g.
   `xterm-256color`) for the remote side, while your local terminal keeps its
   real `TERM`.

2. Open a new shell to pick it up, then confirm the wrapper is live:
   ```bash
   tabbing-doctor
   ```
   The SSH section should now report `tabbing-on ssh shim is active in this shell`.

**Verify:** SSH to a remote host and run `echo $TERM` there — it should print
a widely-supported value, not `xterm-ghostty`/`xterm-kitty`.

**Gotchas:**
- **Still seeing the warning after adding the `eval` line:** you need a *new*
  shell — the wrapper is a shell function defined at shell-rc load time.
- **You already have your own `ssh()` wrapper function:** `tabbing-doctor`
  detects any function or `ssh()` definition containing `ssh` in your rc files
  and treats that as satisfying the check — no need to also add the shim.
- **You don't use Ghostty or Kitty:** `tabbing-doctor` only warns when
  `TERM` is one of these advanced values; standard `TERM`s need no override.
