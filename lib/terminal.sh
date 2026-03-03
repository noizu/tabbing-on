#!/bin/sh
# lib/terminal.sh — Terminal detection and escape sequence abstraction
#
# Detects which terminal emulator is running and provides abstracted
# functions for setting titles, tab colors, and badges.
# Uses POSIX sh for portability.

# ---------------------------------------------------------------------------
# Detect terminal emulator, set TAB_TERMINAL
# Called once at init time; can be re-called to refresh
# ---------------------------------------------------------------------------
_tabbing_detect_terminal() {
  if [ -n "${ITERM_SESSION_ID:-}" ]; then
    TAB_TERMINAL="iterm2"
  elif [ -n "${GHOSTTY_RESOURCES_DIR:-}" ]; then
    TAB_TERMINAL="ghostty"
  elif [ -n "${KITTY_WINDOW_ID:-}" ]; then
    TAB_TERMINAL="kitty"
  elif [ "${TERM_PROGRAM:-}" = "WezTerm" ]; then
    TAB_TERMINAL="wezterm"
  elif [ "${TERM_PROGRAM:-}" = "Apple_Terminal" ]; then
    TAB_TERMINAL="apple-terminal"
  elif [ -n "${WT_SESSION:-}" ]; then
    TAB_TERMINAL="windows-terminal"
  elif [ "${TERM_PROGRAM:-}" = "Alacritty" ] || [ "${TERM:-}" = "alacritty" ]; then
    TAB_TERMINAL="alacritty"
  elif [ -n "${KONSOLE_VERSION:-}" ]; then
    TAB_TERMINAL="konsole"
  elif [ -n "${GNOME_TERMINAL_SCREEN:-}" ]; then
    TAB_TERMINAL="gnome-terminal"
  elif [ -n "${TMUX:-}" ]; then
    TAB_TERMINAL="tmux"
  elif [ "${TERM:-}" = "xterm" ] || [ "${TERM:-}" = "xterm-256color" ]; then
    TAB_TERMINAL="xterm"
  else
    TAB_TERMINAL="unknown"
  fi
  export TAB_TERMINAL
}

# ---------------------------------------------------------------------------
# Send tab/window title via escape sequence
# OSC 0 is universally supported across all target terminals
# ---------------------------------------------------------------------------
_tabbing_send_title() {
  local title="$1"

  case "${TAB_TERMINAL:-unknown}" in
    tmux)
      # tmux: use tmux-specific escape to pass through to outer terminal
      # Also set tmux window name
      printf '\033]0;%s\007' "$title"
      ;;
    *)
      # Universal: OSC 0 — works on all target terminals:
      # iterm2, ghostty, kitty, wezterm, alacritty, apple-terminal,
      # windows-terminal, konsole, gnome-terminal, xterm
      printf '\033]0;%s\007' "$title"
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Set tab background color (terminal-specific, no-op where unsupported)
# Args: r g b (0-255 each)
# ---------------------------------------------------------------------------
_tabbing_send_tab_color() {
  local r="$1" g="$2" b="$3"

  case "${TAB_TERMINAL:-unknown}" in
    iterm2)
      # iTerm2 proprietary: OSC 6;1;bg;{channel};brightness;{value} BEL
      printf '\033]6;1;bg;red;brightness;%s\007' "$r"
      printf '\033]6;1;bg;green;brightness;%s\007' "$g"
      printf '\033]6;1;bg;blue;brightness;%s\007' "$b"
      ;;
    kitty)
      # Kitty remote control (requires allow_remote_control in config)
      if command -v kitty >/dev/null 2>&1; then
        kitty @ set-tab-color "active_bg=#$(printf '%02x%02x%02x' "$r" "$g" "$b")" 2>/dev/null || true
      fi
      ;;
    *)
      # No tab color support — silently skip
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Clear/reset tab color to default (terminal-specific)
# ---------------------------------------------------------------------------
_tabbing_clear_tab_color() {
  case "${TAB_TERMINAL:-unknown}" in
    iterm2)
      printf '\033]6;1;bg;*;default\007'
      ;;
    kitty)
      if command -v kitty >/dev/null 2>&1; then
        kitty @ set-tab-color --reset 2>/dev/null || true
      fi
      ;;
    *)
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Set badge text (terminal-specific, no-op where unsupported)
# Badges are persistent labels shown in the terminal background
# ---------------------------------------------------------------------------
_tabbing_send_badge() {
  local badge="$1"

  case "${TAB_TERMINAL:-unknown}" in
    iterm2)
      # iTerm2: OSC 1337;SetBadgeFormat=<base64> BEL
      local encoded
      if command -v base64 >/dev/null 2>&1; then
        encoded="$(printf '%s' "$badge" | base64)"
        printf '\033]1337;SetBadgeFormat=%s\007' "$encoded"
      fi
      ;;
    *)
      # No badge support — silently skip
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Clear badge (terminal-specific)
# ---------------------------------------------------------------------------
_tabbing_clear_badge() {
  case "${TAB_TERMINAL:-unknown}" in
    iterm2)
      printf '\033]1337;SetBadgeFormat=\007'
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

# ---------------------------------------------------------------------------
# Print detected terminal info
# ---------------------------------------------------------------------------
_tabbing_terminal_info() {
  printf 'Terminal: %s\n' "${TAB_TERMINAL:-unknown}"
  printf 'Features:\n'
  printf '  Title (OSC 0):  yes (universal)\n'
  case "${TAB_TERMINAL:-unknown}" in
    iterm2)
      printf '  Tab Color:      yes (iTerm2 OSC 6)\n'
      printf '  Badge:          yes (iTerm2 OSC 1337)\n'
      ;;
    kitty)
      printf '  Tab Color:      yes (kitty remote control)\n'
      printf '  Badge:          no\n'
      ;;
    *)
      printf '  Tab Color:      no\n'
      printf '  Badge:          no\n'
      ;;
  esac
}
