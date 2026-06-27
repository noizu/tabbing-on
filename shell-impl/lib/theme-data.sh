#!/bin/sh
# lib/theme-data.sh — Line-indexed single-variable theme store.
#
# The entire theme lives in ONE variable (default: TAB_THEME_DATA), a
# newline-delimited block where the *line number* is the lookup key — no YAML,
# no JSON, no yq. Reading a value is just `sed -n "${line}p"`. Pure POSIX.
#
# Line map (1-based):
#
#   Lines 1..107   keyed by the SGR code itself
#                    1..9    attribute toggles (bold=1, underline=4, ...)  -> "on"/blank
#                    30..37  standard foreground colors                    -> #RRGGBB
#                    40..47  standard background colors                    -> #RRGGBB
#                    50..65  special attributes (framed=51, overline=53...) -> "on"/blank
#                    90..97  bright foreground                             -> #RRGGBB
#                    100..107 bright background                            -> #RRGGBB
#   Lines 110..112 OSC ui colors, mnemonic to their OSC number
#                    110 = OSC 10 foreground
#                    111 = OSC 11 background
#                    112 = OSC 12 cursor
#   Lines 128..383 the xterm-256 palette: line = 128 + slot
#                    128..143  slots 0..15   (named ANSI)
#                    144..359  slots 16..231 (6x6x6 cube)
#                    360..383  slots 232..255 (24-step grayscale)
#
# Logical handles map to a line:
#   bright-red          -> 91   (SGR bright-red foreground)
#   bg-blue             -> 44   (SGR background blue)
#   background|fg|cursor-> 111/110/112
#   slot-<N>            -> 128 + N            (0..255)
#   grayscale-<N>       -> 360 + N            (0..23  xterm grayscale ramp slot)
#   THEME_CUBE_COLOR_RRGGBB | cube-RRGGBB
#                       -> 128 + cube_slot(RRGGBB)
#
# Note: `grayscale-N` (0..23) is the xterm grayscale *slot*; the X11 color names
# `gray-N`/`grey-N` (0..100, in data/colors.txt) are a value vocabulary, not
# slot addresses — they resolve to hex via _tabbing_x11_to_hex.
#
# A cube hex reverse-maps to its canonical slot (16 + 36*r + 6*g + b) where each
# channel is one of the six cube levels 00/5f/87/af/d7/ff -> 0..5.

THEME_DATA_LINES=383

# --- channel hex (2 digits) -> cube level index 0..5 --------------------------
_tabbing_cube_level() {
  case "$(printf '%s' "${1:-}" | tr 'A-F' 'a-f')" in
    00) echo 0 ;; 5f) echo 1 ;; 87) echo 2 ;;
    af) echo 3 ;; d7) echo 4 ;; ff) echo 5 ;;
    *)  return 1 ;;   # not a canonical cube channel
  esac
}

# --- cube hex RRGGBB -> palette slot 16..231 ----------------------------------
_tabbing_cube_slot() {
  local hex="${1#\#}" r g b
  [ "${#hex}" -eq 6 ] || return 1
  r="$(_tabbing_cube_level "$(printf '%s' "$hex" | cut -c1-2)")" || return 1
  g="$(_tabbing_cube_level "$(printf '%s' "$hex" | cut -c3-4)")" || return 1
  b="$(_tabbing_cube_level "$(printf '%s' "$hex" | cut -c5-6)")" || return 1
  echo $((16 + 36 * r + 6 * g + b))
}

# --- logical handle -> line number (echo line, or return 1) -------------------
_tabbing_theme_line() {
  local h="${1:-}"
  case "$h" in
    foreground|fg)      echo 110; return 0 ;;
    background|bg)      echo 111; return 0 ;;
    cursor)            echo 112; return 0 ;;
    slot-*)            echo $((128 + ${h#slot-})); return 0 ;;
    grayscale-*)       echo $((360 + ${h#grayscale-})); return 0 ;;
    THEME_CUBE_COLOR_*|cube-*)
      local hx="${h#THEME_CUBE_COLOR_}"; hx="${hx#cube-}"
      local slot; slot="$(_tabbing_cube_slot "$hx")" || return 1
      echo $((128 + slot)); return 0 ;;
    bg-*)
      # background named color: fg code + 10
      local fg; fg="$(_tabbing_color_code "${h#bg-}" 2>/dev/null)" || return 1
      case "$fg" in 3[0-7]|9[0-7]) echo $((fg + 10)); return 0 ;; *) return 1 ;; esac ;;
    [0-9]*)            echo "$h"; return 0 ;;     # raw SGR code / line number
    *)
      # named foreground color or attribute -> its SGR code is the line
      _tabbing_color_code "$h" 2>/dev/null && return 0
      return 1 ;;
  esac
}

# --- pad the data var to THEME_DATA_LINES lines (idempotent) -------------------
_tabbing_theme_data_pad() {
  awk -v n="$THEME_DATA_LINES" '
    { print; c++ }
    END { while (c < n) { print ""; c++ } }'
}

# --- get value at handle from $1=varname (default TAB_THEME_DATA) --------------
_tabbing_theme_get() {
  local handle="$1" var="${2:-TAB_THEME_DATA}" line
  line="$(_tabbing_theme_line "$handle")" || return 1
  eval "printf '%s\n' \"\${$var:-}\"" | sed -n "${line}p"
}

# --- X11/Tk symbolic color name -> #RRGGBB (data/colors.txt) ------------------
_tabbing_x11_data() {
  # locate colors.txt next to the lib dir (handles installed + in-repo layouts)
  local d
  for d in "${TABBING_ROOT:-}/data" \
           "${XDG_DATA_HOME:-$HOME/.local/share}/tabbing-on/data" \
           "$(dirname "$0" 2>/dev/null)/../data"; do
    [ -f "$d/colors.txt" ] && { echo "$d/colors.txt"; return 0; }
  done
  return 1
}

_tabbing_x11_to_hex() {
  local name file
  name="$(printf '%s' "${1:-}" | tr '[:upper:]' '[:lower:]' | sed 's/[[:space:]]\{1,\}/-/g')"
  file="$(_tabbing_x11_data)" || return 1
  awk -F'|' -v n="$name" '$1==n{print $2; found=1; exit} END{exit !found}' "$file"
}

# --- resolve any value (hex passthrough | named ANSI | X11 name) -> #RRGGBB ----
_tabbing_resolve_value() {
  local v="${1:-}"
  case "$v" in
    \#[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]) echo "$v"; return 0 ;;
    on|off|"") echo "$v"; return 0 ;;   # attribute toggles / clear
  esac
  # try the built-in named-color->hex map (render.sh), then the X11 db
  _tabbing_color_to_hex "$v" 2>/dev/null && return 0
  _tabbing_x11_to_hex "$v" && return 0
  echo "$v"   # leave untouched if unrecognized
}

# --- set value at handle: _tabbing_theme_set <handle> <value> [varname] --------
# Value may be #RRGGBB, a named ANSI color, an X11 name (e.g. dodger-blue), or
# on/off for attribute lines. Rewrites the single line in-place; re-exports.
_tabbing_theme_set() {
  local handle="$1" value="$2" var="${3:-TAB_THEME_DATA}" line cur
  line="$(_tabbing_theme_line "$handle")" || return 1
  value="$(_tabbing_resolve_value "$value")"
  eval "cur=\"\${$var:-}\""
  cur="$(printf '%s\n' "$cur" | _tabbing_theme_data_pad \
        | awk -v n="$line" -v v="$value" 'NR==n{print v;next}{print}')"
  eval "$var=\$cur"
  eval "export $var"
}

# --- integer RGB lerp between two #RRGGBB at `steps` evenly-spaced points ------
# Echoes one #RRGGBB per line (inclusive of both endpoints).
_tabbing_gen_ramp() {
  # Resolve named / X11 colors to hex first, else a 6-char name like "purple"
  # parses as zeroed hex channels and yields an all-#000000 ramp.
  local from to steps="${3:-6}"
  from="$(_tabbing_resolve_value "$1")"; from="${from#\#}"
  to="$(_tabbing_resolve_value "$2")"; to="${to#\#}"
  # bail if either endpoint isn't a 6-digit hex (no silent black)
  case "$from" in [0-9A-Fa-f][0-9A-Fa-f][0-9A-Fa-f][0-9A-Fa-f][0-9A-Fa-f][0-9A-Fa-f]) ;; *) return 1 ;; esac
  case "$to"   in [0-9A-Fa-f][0-9A-Fa-f][0-9A-Fa-f][0-9A-Fa-f][0-9A-Fa-f][0-9A-Fa-f]) ;; *) return 1 ;; esac
  # convert hex channels in-shell (portable; awk strtonum is gawk-only)
  local ar ag ab br bg bb
  ar=$(printf '%d' "0x$(printf %s "$from" | cut -c1-2)")
  ag=$(printf '%d' "0x$(printf %s "$from" | cut -c3-4)")
  ab=$(printf '%d' "0x$(printf %s "$from" | cut -c5-6)")
  br=$(printf '%d' "0x$(printf %s "$to" | cut -c1-2)")
  bg=$(printf '%d' "0x$(printf %s "$to" | cut -c3-4)")
  bb=$(printf '%d' "0x$(printf %s "$to" | cut -c5-6)")
  awk -v ar="$ar" -v ag="$ag" -v ab="$ab" -v br="$br" -v bg="$bg" -v bb="$bb" -v n="$steps" 'BEGIN{
    if (n < 2) n = 2
    for (i = 0; i < n; i++) {
      t = i / (n - 1)
      printf "#%02X%02X%02X\n", ar+(br-ar)*t+0.5, ag+(bg-ag)*t+0.5, ab+(bb-ab)*t+0.5
    }
  }'
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

# --- standard xterm-256 values for slots 16..255 -> blob lines 144..383 --------
# Fills the 6x6x6 cube and 24-step grayscale with canonical xterm values so a
# 256-color app renders correctly. Operates on $1=varname (default TAB_THEME_DATA).
_tabbing_gen_standard_palette() {
  local var="${1:-TAB_THEME_DATA}" cur
  eval "cur=\"\${$var:-}\""
  cur="$(printf '%s\n' "$cur" | _tabbing_theme_data_pad | awk 'BEGIN{
      split("0 95 135 175 215 255", lv, " ")
    }
    {
      n = NR
      if (n >= 144 && n <= 359) {                 # cube slots 16..231
        s = n - 128; idx = s - 16
        r = int(idx / 36); g = int((idx % 36) / 6); b = idx % 6
        printf "#%02X%02X%02X\n", lv[r+1], lv[g+1], lv[b+1]
      } else if (n >= 360 && n <= 383) {           # grayscale slots 232..255
        g = (n - 360) * 10 + 8
        printf "#%02X%02X%02X\n", g, g, g
      } else print
    }')"
  eval "$var=\$cur"
  eval "export $var"
}
