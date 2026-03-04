#!/bin/sh
# lib/core.sh — Supplementary core functions for tabbing-on
#
# Contains functions NOT needed by the prompt hook or adapter:
# emoji list, color list, help text, YAML escaping.
#
# The render pipeline (color_code, emoji_lookup, render, display, etc.)
# has been moved to lib/render.sh for minimal adapter footprint.

# ---------------------------------------------------------------------------
# Print all available emojis with names
# ---------------------------------------------------------------------------
_tabbing_emoji_list() {
  printf 'Available emojis:\n'
  printf '\n  Build & Deploy:\n'
  printf '    rocket       %s    ship         %s    package      %s\n' \
    "$(_tabbing_emoji_lookup rocket)" "$(_tabbing_emoji_lookup ship)" "$(_tabbing_emoji_lookup package)"
  printf '    construction %s    hammer       %s\n' \
    "$(_tabbing_emoji_lookup construction)" "$(_tabbing_emoji_lookup hammer)"

  printf '\n  Status:\n'
  printf '    check/done   %s    cross/fail   %s    warning/warn %s\n' \
    "$(_tabbing_emoji_lookup check)" "$(_tabbing_emoji_lookup cross)" "$(_tabbing_emoji_lookup warning)"
  printf '    stop         %s    hourglass    %s\n' \
    "$(_tabbing_emoji_lookup stop)" "$(_tabbing_emoji_lookup hourglass)"

  printf '\n  Activity:\n'
  printf '    bug          %s    fire         %s    test/lab     %s\n' \
    "$(_tabbing_emoji_lookup bug)" "$(_tabbing_emoji_lookup fire)" "$(_tabbing_emoji_lookup test)"
  printf '    search/mag   %s    wrench/fix   %s    gear/config  %s\n' \
    "$(_tabbing_emoji_lookup search)" "$(_tabbing_emoji_lookup wrench)" "$(_tabbing_emoji_lookup gear)"
  printf '    lock/secure  %s    key          %s    trash/delete %s\n' \
    "$(_tabbing_emoji_lookup lock)" "$(_tabbing_emoji_lookup key)" "$(_tabbing_emoji_lookup trash)"

  printf '\n  Review & Comms:\n'
  printf '    eyes/review  %s    chat/discuss %s    mail/email   %s\n' \
    "$(_tabbing_emoji_lookup eyes)" "$(_tabbing_emoji_lookup chat)" "$(_tabbing_emoji_lookup mail)"
  printf '    bell/alert   %s\n' "$(_tabbing_emoji_lookup bell)"

  printf '\n  Data & Infra:\n'
  printf '    db/database  %s    cloud        %s    link         %s\n' \
    "$(_tabbing_emoji_lookup db)" "$(_tabbing_emoji_lookup cloud)" "$(_tabbing_emoji_lookup link)"
  printf '    electric/zap %s\n' "$(_tabbing_emoji_lookup electric)"

  printf '\n  Progress:\n'
  printf '    sparkle      %s    star         %s    coffee/break %s\n' \
    "$(_tabbing_emoji_lookup sparkle)" "$(_tabbing_emoji_lookup star)" "$(_tabbing_emoji_lookup coffee)"
  printf '    sleep/zzz    %s    brain/think  %s    books/docs   %s\n' \
    "$(_tabbing_emoji_lookup sleep)" "$(_tabbing_emoji_lookup brain)" "$(_tabbing_emoji_lookup books)"
  printf '    pin          %s    clipboard    %s    chart        %s\n' \
    "$(_tabbing_emoji_lookup pin)" "$(_tabbing_emoji_lookup clipboard)" "$(_tabbing_emoji_lookup chart)"

  printf '\n  Miscellaneous:\n'
  printf '    tada         %s    art/design   %s    bulb/idea    %s\n' \
    "$(_tabbing_emoji_lookup tada)" "$(_tabbing_emoji_lookup art)" "$(_tabbing_emoji_lookup bulb)"
  printf '    shield       %s    recycle      %s    truck/move   %s\n' \
    "$(_tabbing_emoji_lookup shield)" "$(_tabbing_emoji_lookup recycle)" "$(_tabbing_emoji_lookup truck)"
  printf '    memo/note    %s\n' "$(_tabbing_emoji_lookup memo)"
  printf '\n'
}

# ---------------------------------------------------------------------------
# Print available colors
# ---------------------------------------------------------------------------
_tabbing_color_list() {
  printf 'Available colors:\n\n'
  printf '  Standard:  black  red  green  yellow  blue  magenta  cyan  white\n'
  printf '  Bright:    bright-black  bright-red  bright-green  bright-yellow\n'
  printf '             bright-blue  bright-magenta  bright-cyan  bright-white\n'
  printf '  Effects:   bold  dim  italic  underline  blink  inverse  strikethrough\n'
  printf '  Raw:       any ANSI code (e.g. 31, 91, 196)\n'
  printf '\n'
}

# ---------------------------------------------------------------------------
# Help / usage
# ---------------------------------------------------------------------------
_tabbing_help() {
  printf 'tabbing-on — Terminal tab title, status & task manager\n\n'
  printf 'Usage:\n'
  printf '  tabbing-on [flags] "title" "status"    Set tab title and status\n'
  printf '  tabbing-on                             Show current state\n'
  printf '  tabbing-on emojis                      List available emojis\n'
  printf '  tabbing-on colors                      List available colors\n'
  printf '  tabbing-on help                        Show this help\n'
  printf '\n'
  printf 'Flags:\n'
  printf '  --highlight COLOR, -h COLOR, -COLOR    Set title highlight color\n'
  printf '  --urgency N, --pri N, -p N, -priN      Set urgency 0-5 (0=critical)\n'
  printf '  --emoji NAME, -e NAME, -NAME           Set emoji indicator\n'
  printf '  --no-emoji                             Clear emoji\n'
  printf '  --record                               Start asciinema recording\n'
  printf '  --continue                             Keep recording across status change\n'
  printf '  --stop-recording                       Stop recording\n'
  printf '  --terminal-info                        Show detected terminal\n'
  printf '\n'
  printf 'Related commands:\n'
  printf '  tabbing-status [flags] "text"           Update status only\n'
  printf '  tabbing-todo "title" [-m "desc"]        Add todo item\n'
  printf '  tabbing-todo                            List todos\n'
  printf '  tabbing-todo --pick                     Switch to a todo interactively\n'
  printf '  tabbing-todo --done [id]                Mark todo done\n'
  printf '  tabbing-report [--mermaid] [--all]      Time-in-state report\n'
  printf '  tabbing-history [query]                 Search/browse history\n'
  printf '  tabbing-info                             Full tab info (state, paths, recordings)\n'
  printf '  tabbing-recordings                      List recordings\n'
  printf '  tabbing-clear history|todos|recordings   Clear data\n'
  printf '\n'
  printf 'Examples:\n'
  printf '  tabbing-on -rocket -blue "MyApp" "deploying" -p 2\n'
  printf '  tabbing-status -fire "hotfix"\n'
  printf '  tabbing-todo "Write tests" -e test -m "Cover edge cases"\n'
  printf '\n'
}

# ---------------------------------------------------------------------------
# YAML-safe string escaping
# ---------------------------------------------------------------------------
_tabbing_yaml_escape() {
  printf '%s' "$1" | sed 's/"/\\"/g'
}
