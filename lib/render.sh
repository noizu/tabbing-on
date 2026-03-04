#!/bin/sh
# lib/render.sh — Minimal render pipeline for prompt hook + validation
#
# Contains ONLY the functions needed by the shell adapter:
# - Prompt hook rendering (title + urgency color)
# - Color/emoji validation (for arg parsing)
# - Display (for no-args state display)
#
# Sourced by shell adapters at init time. All other functions
# remain in core.sh/terminal.sh and are only loaded by bin/ scripts.

# ---------------------------------------------------------------------------
# Color name → ANSI code mapping
# ---------------------------------------------------------------------------
_tabbing_color_code() {
  case "${1:-}" in
    black)          echo "30" ;;
    red)            echo "31" ;;
    green)          echo "32" ;;
    yellow)         echo "33" ;;
    blue)           echo "34" ;;
    magenta)        echo "35" ;;
    cyan)           echo "36" ;;
    white)          echo "37" ;;
    bright-black)   echo "90" ;;
    bright-red)     echo "91" ;;
    bright-green)   echo "92" ;;
    bright-yellow)  echo "93" ;;
    bright-blue)    echo "94" ;;
    bright-magenta) echo "95" ;;
    bright-cyan)    echo "96" ;;
    bright-white)   echo "97" ;;
    bold)           echo "1" ;;
    dim)            echo "2" ;;
    italic)         echo "3" ;;
    underline)      echo "4" ;;
    blink)          echo "5" ;;
    inverse)        echo "7" ;;
    strikethrough)  echo "9" ;;
    [0-9]*)         echo "$1" ;;
    *)
      echo "tabbing: unknown color/effect '$1'" >&2
      echo "tabbing: available: black red green yellow blue magenta cyan white" >&2
      echo "tabbing:   bright-{black,red,green,yellow,blue,magenta,cyan,white}" >&2
      echo "tabbing:   bold dim italic underline blink inverse strikethrough" >&2
      echo "tabbing:   or raw ANSI codes (e.g. 31, 91)" >&2
      return 1
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Check if a string is a known color name (replaces array lookup)
# ---------------------------------------------------------------------------
_tabbing_is_known_color() {
  case "${1:-}" in
    black|red|green|yellow|blue|magenta|cyan|white) return 0 ;;
    bright-black|bright-red|bright-green|bright-yellow) return 0 ;;
    bright-blue|bright-magenta|bright-cyan|bright-white) return 0 ;;
    bold|dim|italic|underline|blink|inverse|strikethrough) return 0 ;;
    *) return 1 ;;
  esac
}

# ---------------------------------------------------------------------------
# Urgency palette: 0=critical(red) → 5=nominal(green/gray)
# ---------------------------------------------------------------------------
_tabbing_urgency_ansi() {
  case "${1:-}" in
    0) echo "91" ;;  # bright red
    1) echo "31" ;;  # red
    2) echo "33" ;;  # yellow
    3) echo "93" ;;  # bright yellow
    4) echo "32" ;;  # green
    5) echo "90" ;;  # gray
    *) echo "0"  ;;  # default
  esac
}

_tabbing_urgency_dot() {
  case "${1:-}" in
    0) printf '\xF0\x9F\x94\xB4' ;;  # red circle
    1) printf '\xF0\x9F\x9F\xA0' ;;  # orange circle
    2) printf '\xF0\x9F\x9F\xA1' ;;  # yellow circle
    3) printf '\xF0\x9F\x9F\xA2' ;;  # green circle
    4) printf '\xF0\x9F\x9F\xA2' ;;  # green circle
    5) printf '\xE2\x9A\xAA'     ;;  # white/gray circle
    *) ;;
  esac
}

# ---------------------------------------------------------------------------
# Emoji lookup: name → Unicode character
# Uses raw UTF-8 bytes for bash 3.2 compatibility
# ---------------------------------------------------------------------------
_tabbing_emoji_lookup() {
  case "${1:-}" in
    # --- Build & Deploy ---
    rocket)              printf '\xF0\x9F\x9A\x80' ;;
    ship)                printf '\xF0\x9F\x9A\xA2' ;;
    package)             printf '\xF0\x9F\x93\xA6' ;;
    construction)        printf '\xF0\x9F\x9A\xA7' ;;
    hammer)              printf '\xF0\x9F\x94\xA8' ;;

    # --- Status ---
    check|done)          printf '\xE2\x9C\x85'     ;;
    cross|fail)          printf '\xE2\x9D\x8C'     ;;
    warning|warn)        printf '\xE2\x9A\xA0'     ;;
    stop)                printf '\xF0\x9F\x9B\x91' ;;
    hourglass|wait)      printf '\xE2\x8F\xB3'     ;;

    # --- Activity ---
    bug)                 printf '\xF0\x9F\x90\x9B' ;;
    fire)                printf '\xF0\x9F\x94\xA5' ;;
    test|lab)            printf '\xF0\x9F\xA7\xAA' ;;
    search|mag)          printf '\xF0\x9F\x94\x8D' ;;
    wrench|fix)          printf '\xF0\x9F\x94\xA7' ;;
    gear|config)         printf '\xE2\x9A\x99'     ;;
    lock|secure)         printf '\xF0\x9F\x94\x92' ;;
    key)                 printf '\xF0\x9F\x94\x91' ;;
    trash|delete)        printf '\xF0\x9F\x97\x91' ;;

    # --- Communication / Review ---
    eyes|review)         printf '\xF0\x9F\x91\x80' ;;
    chat|discuss)        printf '\xF0\x9F\x92\xAC' ;;
    mail|email)          printf '\xF0\x9F\x93\xA7' ;;
    bell|alert)          printf '\xF0\x9F\x94\x94' ;;

    # --- Data / Infra ---
    db|database)         printf '\xF0\x9F\x97\x84' ;;
    cloud)               printf '\xE2\x98\x81'     ;;
    link)                printf '\xF0\x9F\x94\x97' ;;
    electric|zap)        printf '\xE2\x9A\xA1'     ;;

    # --- Progress ---
    sparkle|clean)       printf '\xE2\x9C\xA8'     ;;
    star)                printf '\xE2\xAD\x90'     ;;
    coffee|break)        printf '\xE2\x98\x95'     ;;
    sleep|zzz)           printf '\xF0\x9F\x92\xA4' ;;
    brain|think)         printf '\xF0\x9F\xA7\xA0' ;;
    books|docs)          printf '\xF0\x9F\x93\x9A' ;;
    pin)                 printf '\xF0\x9F\x93\x8C' ;;
    clipboard)           printf '\xF0\x9F\x93\x8B' ;;
    chart)               printf '\xF0\x9F\x93\x8A' ;;

    # --- Miscellaneous ---
    tada|celebrate)      printf '\xF0\x9F\x8E\x89' ;;
    art|design)          printf '\xF0\x9F\x8E\xA8' ;;
    bulb|idea)           printf '\xF0\x9F\x92\xA1' ;;
    shield|protect)      printf '\xF0\x9F\x9B\xA1' ;;
    recycle|refactor)    printf '\xE2\x99\xBB'     ;;
    truck|move)          printf '\xF0\x9F\x9A\x9A' ;;
    memo|note)           printf '\xF0\x9F\x93\x9D' ;;

    *)
      printf "tabbing: unknown emoji '%s'\n" "$1" >&2
      printf "tabbing: use 'tabbing-on --emoji-list' to see available emojis\n" >&2
      return 1
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Check if a string is a known emoji name
# ---------------------------------------------------------------------------
_tabbing_is_known_emoji() {
  case "${1:-}" in
    rocket|ship|package|construction|hammer) return 0 ;;
    check|done|cross|fail|warning|warn|stop|hourglass|wait) return 0 ;;
    bug|fire|test|lab|search|mag|wrench|fix|gear|config) return 0 ;;
    lock|secure|key|trash|delete) return 0 ;;
    eyes|review|chat|discuss|mail|email|bell|alert) return 0 ;;
    db|database|cloud|link|electric|zap) return 0 ;;
    sparkle|clean|star|coffee|break|sleep|zzz|brain|think) return 0 ;;
    books|docs|pin|clipboard|chart) return 0 ;;
    tada|celebrate|art|design|bulb|idea|shield|protect) return 0 ;;
    recycle|refactor|truck|move|memo|note) return 0 ;;
    *) return 1 ;;
  esac
}

# ---------------------------------------------------------------------------
# Internal: get the leading dot/emoji for tab title
# TAB_EMOJI overrides urgency dot when set
# ---------------------------------------------------------------------------
_tabbing_get_indicator() {
  if [ -n "${TAB_EMOJI:-}" ]; then
    _tabbing_emoji_lookup "$TAB_EMOJI"
  elif [ -n "${TAB_URGENCY:-}" ]; then
    _tabbing_urgency_dot "$TAB_URGENCY"
  fi
}

# ---------------------------------------------------------------------------
# Internal: render tab title from TAB_TITLE + TAB_STATUS
# Calls _tabbing_send_title (below)
# ---------------------------------------------------------------------------
_tabbing_render() {
  local max_title=25
  local max_status=20
  local t_title="${TAB_TITLE:-}"
  local t_status="${TAB_STATUS:-}"

  # Truncate title
  if [ ${#t_title} -gt $max_title ]; then
    t_title="$(printf '%s' "$t_title" | cut -c1-$((max_title - 1)))"
    t_title="${t_title}$(printf '\xE2\x80\xA6')"  # ...
  fi

  # Indicator (emoji or urgency dot)
  local dot=""
  dot="$(_tabbing_get_indicator)"
  if [ -n "$dot" ]; then
    dot="$dot "
  fi

  local output="${dot}${t_title}"

  if [ -n "$t_status" ]; then
    # Truncate status
    if [ ${#t_status} -gt $max_status ]; then
      t_status="$(printf '%s' "$t_status" | cut -c1-$((max_status - 1)))"
      t_status="${t_status}$(printf '\xE2\x80\xA6')"  # ...
    fi
    output="${dot}${t_title}: ${t_status}"
  fi

  _tabbing_send_title "$output"
}

# ---------------------------------------------------------------------------
# Internal: colored console display of current state
# ---------------------------------------------------------------------------
_tabbing_display() {
  local reset
  reset="$(printf '\033[0m')"

  # Title highlight
  local tc="$reset"
  if [ -n "${TAB_HIGHLIGHT:-}" ]; then
    local code
    code="$(_tabbing_color_code "$TAB_HIGHLIGHT" 2>/dev/null)"
    if [ -n "$code" ]; then
      tc="$(printf '\033[%sm' "$code")"
    fi
  fi

  # Status color (from urgency)
  local sc="$reset"
  if [ -n "${TAB_URGENCY:-}" ]; then
    sc="$(printf '\033[%sm' "$(_tabbing_urgency_ansi "$TAB_URGENCY")")"
  fi

  # Indicator
  local dot=""
  dot="$(_tabbing_get_indicator)"
  if [ -n "$dot" ]; then
    dot="$dot "
  fi

  local title_text="${TAB_TITLE:-(not set)}"
  local line="${tc}${title_text}${reset}"
  if [ -n "${TAB_STATUS:-}" ]; then
    line="${line}: ${dot}${sc}${TAB_STATUS}${reset}"
  fi

  printf '%s\n' "$line"
}

# ---------------------------------------------------------------------------
# Send tab/window title via escape sequence
# OSC 0 is universally supported across all target terminals
# Writes to /dev/tty to avoid interfering with shell integration
# hooks (Ghostty, Kitty) that also write escape sequences during
# prompt rendering. Stdout output in precmd can corrupt cursor state.
# ---------------------------------------------------------------------------
_tabbing_send_title() {
  local title="$1"

  # Prevent Claude Code from overwriting our title
  export CLAUDE_CODE_DISABLE_TERMINAL_TITLE=1

  printf '\033]0;%s\007' "$title" >/dev/tty 2>/dev/null || \
    printf '\033]0;%s\007' "$title"
}

# ---------------------------------------------------------------------------
# Clear tab/window title — sends empty OSC 2 to reset terminal title state
# On Ghostty this resets `seen_title = false`, re-enabling the pwd fallback.
# Uses OSC 2 (not OSC 0) to specifically target the title without icon name.
# ---------------------------------------------------------------------------
_tabbing_clear_title() {
  printf '\033]2;\007' >/dev/tty 2>/dev/null || \
    printf '\033]2;\007'
}

# ---------------------------------------------------------------------------
# Set tab background color (terminal-specific, no-op where unsupported)
# Args: r g b (0-255 each)
# ---------------------------------------------------------------------------
_tabbing_send_tab_color() {
  local r="$1" g="$2" b="$3"

  case "${TAB_TERMINAL:-unknown}" in
    iterm2)
      {
        printf '\033]6;1;bg;red;brightness;%s\007' "$r"
        printf '\033]6;1;bg;green;brightness;%s\007' "$g"
        printf '\033]6;1;bg;blue;brightness;%s\007' "$b"
      } >/dev/tty 2>/dev/null || {
        printf '\033]6;1;bg;red;brightness;%s\007' "$r"
        printf '\033]6;1;bg;green;brightness;%s\007' "$g"
        printf '\033]6;1;bg;blue;brightness;%s\007' "$b"
      }
      ;;
    kitty)
      if command -v kitty >/dev/null 2>&1; then
        kitty @ set-tab-color "active_bg=#$(printf '%02x%02x%02x' "$r" "$g" "$b")" 2>/dev/null || true
      fi
      ;;
    *)
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Map urgency level to tab color RGB values
# Returns: "r g b" or empty if no color mapping
# ---------------------------------------------------------------------------
_tabbing_urgency_tab_color() {
  case "${1:-}" in
    0) echo "200 50 50"   ;;  # critical: red
    1) echo "200 100 50"  ;;  # high: orange-red
    2) echo "200 180 50"  ;;  # medium: yellow
    3) echo "150 200 50"  ;;  # medium-low: yellow-green
    4) echo "50 180 50"   ;;  # low: green
    5) echo "120 120 120" ;;  # nominal: gray
    *) ;;
  esac
}

# ---------------------------------------------------------------------------
# Apply urgency-based tab color (where supported)
# ---------------------------------------------------------------------------
_tabbing_apply_urgency_color() {
  local rgb
  rgb="$(_tabbing_urgency_tab_color "${TAB_URGENCY:-}")"
  if [ -n "$rgb" ]; then
    # shellcheck disable=SC2086
    _tabbing_send_tab_color $rgb
  fi
}
