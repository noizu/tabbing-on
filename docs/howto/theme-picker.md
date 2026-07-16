# How to: browse, apply, and customize terminal color themes

**Goal:** Recolor your whole terminal (background, foreground, cursor, 16-color
palette) — not just the tab title — using a built-in theme or your own.
**Prereqs:** A terminal supporting OSC 4/10/11/12 (Ghostty, iTerm2, Kitty, WezTerm, xterm — see README's Terminal Support matrix).

1. Launch the interactive picker (no args):
   ```bash
   tabbing-theme
   ```
   Arrow keys / `hjkl` to browse, `/` to search by name, `Enter` to apply,
   `e` to open the HSV color-wheel editor, `c` to clone a theme, `d` to
   delete a user theme, `r` to reset, `q`/`Esc` to quit and restore.

2. Or apply directly by name from a script:
   ```bash
   tabbing-theme apply dracula
   tabbing-on "Prod" --theme=danger      # semantic theme, applied via tabbing-on directly
   ```

3. List everything available, or the standard prompt layouts a theme can set:
   ```bash
   tabbing-theme list
   tabbing-theme layouts
   ```

4. Reset to terminal defaults:
   ```bash
   tabbing-theme reset
   # or: tabbing-on --no-theme
   ```

**Verify:** your terminal background/palette actually changes; `tabbing-info`
shows `theme:` set to the applied name.

**Gotchas:**
- **Custom themes aren't showing up:** user `.theme` files must live in
  `~/.config/tabbing-on/themes/` (or `$XDG_CONFIG_HOME/tabbing-on/themes/`).
  Start from a template: `tabbing-theme clone dracula my-dark`, then
  `tabbing-theme edit my-dark`.
- **Only `bg`/`fg` set, rest looks wrong:** those two are the only required
  keys — everything else (cursor, 16-color palette, PS1 layout, semantic
  urgency colors) falls back to sane defaults. See the full key reference and
  an annotated example in the main [README's Custom Themes section](../../README.md#custom-themes).
- **`tabbing-theme save` prints "not yet wired from CLI":** this subcommand is
  a stub — writing the active theme into a project's `.envrc` isn't
  implemented yet. Set `TAB_THEME=<name>` in `.envrc` by hand instead (see
  [howto/dc-mode-and-direnv.md](dc-mode-and-direnv.md)).
