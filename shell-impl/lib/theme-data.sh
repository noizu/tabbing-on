#!/bin/sh
# lib/theme-data.sh — Line-indexed single-variable theme store (shell helpers).
#
# The entire theme lives in ONE variable (default: TAB_THEME_DATA), a
# newline-delimited block where the *line number* is the lookup key — no YAML,
# no JSON, no yq. See docs/theme-data-format.md for the full line map.
#
# VALUE COMPUTATION LIVES IN RUST. The line map, handle→line resolution, value
# resolution (named/X11/hex), the 6x6x6 cube math, gen-ramp, gen-standard, and
# get/set now live in rust/src/theme_data.rs and are reached from the shell
# exclusively through the `tabbing-theme` binary (get/set/gen-ramp/gen-standard/
# x11/emit), overridable via $TABBING_THEME_BIN. The public `tabbing-theme`
# shell function delegates straight to that binary, so those primitives were
# duplicated here with no shell caller — they have been removed to keep a
# single source of truth.
#
# What remains here is the small set of pure-shell helpers the interactive
# theme *apply* path still uses directly:
#   - _tabbing_theme_data_pad     pad the blob to THEME_DATA_LINES lines
#   - _tabbing_theme_attr_codes   enabled SGR attribute codes for the title
#   - _tabbing_{save,load}_theme_data  persist/restore a named .themedata blob

THEME_DATA_LINES=383

# --- pad the data var to THEME_DATA_LINES lines (idempotent) -------------------
_tabbing_theme_data_pad() {
  awk -v n="$THEME_DATA_LINES" '
    { print; c++ }
    END { while (c < n) { print ""; c++ } }'
}

# --- enabled SGR attribute codes (lines 1..9, 50..65 == "on") -> "1;4;53" -----
_tabbing_theme_attr_codes() {
  printf '%s\n' "${TAB_THEME_DATA:-}" | awk '
    ((NR>=1 && NR<=9) || (NR>=50 && NR<=65)) && $0=="on" {
      codes = codes (codes=="" ? "" : ";") NR
    }
    END { print codes }'
}

# --- persistent .themedata files (the line-indexed blob, saved verbatim) ------
_tabbing_themedata_file() { echo "$(_tabbing_theme_dir)/${1}.themedata"; }

_tabbing_save_theme_data() {
  local dir; dir="$(_tabbing_theme_dir)"; mkdir -p "$dir" 2>/dev/null
  printf '%s\n' "${TAB_THEME_DATA:-}" | _tabbing_theme_data_pad > "$(_tabbing_themedata_file "$1")"
}

_tabbing_load_theme_data() {
  local f; f="$(_tabbing_themedata_file "$1")"
  [ -f "$f" ] || return 1
  TAB_THEME_DATA="$(cat "$f")"; export TAB_THEME_DATA
  _tabbing_emit_theme_data
}
