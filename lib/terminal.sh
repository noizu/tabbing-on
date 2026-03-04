#!/bin/sh
# lib/terminal.sh — Terminal detection and supplementary escape sequences
#
# The core render pipeline (send_title, send_tab_color, urgency_tab_color,
# apply_urgency_color) has been moved to lib/render.sh.
#
# This file retains: terminal detection, badge, clear, and info functions
# that are only needed by bin/ scripts, not the prompt hook.

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
# Clear/reset tab color to default (terminal-specific)
# ---------------------------------------------------------------------------
_tabbing_clear_tab_color() {
  case "${TAB_TERMINAL:-unknown}" in
    iterm2)
      printf '\033]6;1;bg;*;default\007' >/dev/tty 2>/dev/null || \
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
      local encoded
      if command -v base64 >/dev/null 2>&1; then
        encoded="$(printf '%s' "$badge" | base64)"
        printf '\033]1337;SetBadgeFormat=%s\007' "$encoded" >/dev/tty 2>/dev/null || \
          printf '\033]1337;SetBadgeFormat=%s\007' "$encoded"
      fi
      ;;
    *)
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Clear badge (terminal-specific)
# ---------------------------------------------------------------------------
_tabbing_clear_badge() {
  case "${TAB_TERMINAL:-unknown}" in
    iterm2)
      printf '\033]1337;SetBadgeFormat=\007' >/dev/tty 2>/dev/null || \
        printf '\033]1337;SetBadgeFormat=\007'
      ;;
    *)
      ;;
  esac
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
