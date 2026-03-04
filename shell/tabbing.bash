#!/bin/bash
# shell/tabbing.bash — Bash-specific tabbing-on command definitions (thin adapter)
#
# Sources ONLY lib/render.sh for minimal namespace footprint.
# Heavy operations (history, recording, todo, session) are delegated
# to bin/ scripts running in subprocesses.
#
# Init: eval "$(tabbing-init bash)" in your .bashrc

# ---------------------------------------------------------------------------
# Locate root and source minimal render pipeline
# ---------------------------------------------------------------------------
_tabbing_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

source "${_tabbing_root}/lib/render.sh"

# ---------------------------------------------------------------------------
# Inline: detect terminal emulator (runs once at init)
# ---------------------------------------------------------------------------
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

# ---------------------------------------------------------------------------
# Inline: generate session fingerprint (runs once at init)
# ---------------------------------------------------------------------------
if [ -z "${TAB_SESSION:-}" ]; then
  if [ -r /dev/urandom ]; then
    TAB_SESSION="$(od -An -tx1 -N4 /dev/urandom 2>/dev/null | tr -d ' \n')"
  fi
  if [ -z "${TAB_SESSION:-}" ]; then
    TAB_SESSION="$(printf '%04x%04x' $$ "$(date +%s)" | cut -c1-8)"
  fi
  export TAB_SESSION
fi

# Known color names for -color shorthand matching (bash array)
_tabbing_known_colors=(
  black red green yellow blue magenta cyan white
  bright-black bright-red bright-green bright-yellow
  bright-blue bright-magenta bright-cyan bright-white
  bold dim italic underline blink inverse strikethrough
)

# ---------------------------------------------------------------------------
# Internal: commit side effects (history, display, session save)
# Uses full libs if available (bin/ context), else delegates to subprocess
# ---------------------------------------------------------------------------
_tabbing_commit() {
  if [ -n "${_TABBING_FULL_LIBS:-}" ]; then
    _tabbing_record_event "${1:-status}"
    _tabbing_display
    _tabbing_session_save
  else
    "${TABBING_ROOT:-$_tabbing_root}/bin/_tabbing-commit" "${1:-status}"
  fi
}

# ---------------------------------------------------------------------------
# tabbing-on [flags in any position] [title] [status]
# ---------------------------------------------------------------------------
tabbing-on() {
  local opt_highlight="" opt_urgency="" opt_emoji=""
  local has_highlight=0 has_urgency=0 has_emoji=0
  local has_record=0 has_continue=0 has_stop_recording=0
  local no_emoji=0
  local positional=()

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --highlight=*) opt_highlight="${1#--highlight=}"; has_highlight=1; shift ;;
      --highlight)   opt_highlight="$2"; has_highlight=1; shift 2 ;;
      --color=*)     opt_highlight="${1#--color=}"; has_highlight=1; shift ;;
      --color)       opt_highlight="$2"; has_highlight=1; shift 2 ;;
      -h)            opt_highlight="$2"; has_highlight=1; shift 2 ;;

      --urgency=*)   opt_urgency="${1#--urgency=}"; has_urgency=1; shift ;;
      --urgency)     opt_urgency="$2"; has_urgency=1; shift 2 ;;
      --pri=*)       opt_urgency="${1#--pri=}"; has_urgency=1; shift ;;
      --pri)         opt_urgency="$2"; has_urgency=1; shift 2 ;;
      -p)            opt_urgency="$2"; has_urgency=1; shift 2 ;;
      -pri[0-5])     opt_urgency="${1#-pri}"; has_urgency=1; shift ;;

      --emoji=*)     opt_emoji="${1#--emoji=}"; has_emoji=1; shift ;;
      --emoji)       opt_emoji="$2"; has_emoji=1; shift 2 ;;
      -e)            opt_emoji="$2"; has_emoji=1; shift 2 ;;
      --no-emoji)    no_emoji=1; shift ;;

      # Subcommands: use full libs if available, else delegate to bin/
      --emoji-list|--emojis)
        if [ -n "${_TABBING_FULL_LIBS:-}" ]; then _tabbing_emoji_list
        else "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-on" --emoji-list; fi
        return ;;
      --help)
        if [ -n "${_TABBING_FULL_LIBS:-}" ]; then _tabbing_help
        else "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-on" --help; fi
        return ;;
      --terminal-info)
        if [ -n "${_TABBING_FULL_LIBS:-}" ]; then _tabbing_terminal_info
        else "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-on" --terminal-info; fi
        return ;;

      --record)         has_record=1; shift ;;
      --continue)       has_continue=1; shift ;;
      --stop-recording) has_stop_recording=1; shift ;;

      # -COLOR or -EMOJI shorthand (e.g. -blue, -rocket)
      -[a-z]*)
        local maybe="${1#-}"
        if [[ "$maybe" =~ ^pri[0-5]$ ]]; then
          opt_urgency="${maybe#pri}"; has_urgency=1; shift
        else
          local found=0
          for c in "${_tabbing_known_colors[@]}"; do
            if [[ "$maybe" == "$c" ]]; then
              opt_highlight="$maybe"; has_highlight=1; found=1
              break
            fi
          done
          if [[ $found -eq 0 ]]; then
            if _tabbing_is_known_emoji "$maybe"; then
              opt_emoji="$maybe"; has_emoji=1; found=1
            fi
          fi
          if [[ $found -eq 1 ]]; then
            shift
          else
            positional+=("$1"); shift
          fi
        fi
        ;;

      *) positional+=("$1"); shift ;;
    esac
  done

  # No args at all — display current state
  if [[ ${#positional[@]} -eq 0 && $has_highlight -eq 0 && $has_urgency -eq 0 && $has_emoji -eq 0 && $no_emoji -eq 0 && $has_record -eq 0 && $has_continue -eq 0 && $has_stop_recording -eq 0 ]]; then
    _tabbing_display
    return
  fi

  # Subcommands: "tabbing-on emojis", "tabbing-on help", "tabbing-on colors"
  if [[ ${#positional[@]} -eq 1 ]]; then
    case "${positional[0]}" in
      emojis|emoji-list)
        if [ -n "${_TABBING_FULL_LIBS:-}" ]; then _tabbing_emoji_list
        else "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-on" --emoji-list; fi
        return ;;
      colors|color-list)
        if [ -n "${_TABBING_FULL_LIBS:-}" ]; then _tabbing_color_list
        else "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-on" colors; fi
        return ;;
      help)
        if [ -n "${_TABBING_FULL_LIBS:-}" ]; then _tabbing_help
        else "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-on" --help; fi
        return ;;
    esac
  fi

  # Validate and apply flags
  if [[ $has_highlight -eq 1 ]]; then
    if ! _tabbing_color_code "$opt_highlight" >/dev/null 2>&1; then
      _tabbing_color_code "$opt_highlight"
      return 1
    fi
    export TAB_HIGHLIGHT="$opt_highlight"
  fi

  if [[ $has_urgency -eq 1 ]]; then
    if [[ ! "$opt_urgency" =~ ^[0-5]$ ]]; then
      echo "tabbing: urgency must be 0-5 (0=critical, 5=nominal)" >&2
      return 1
    fi
    export TAB_URGENCY="$opt_urgency"
  fi

  if [[ $has_emoji -eq 1 ]]; then
    if ! _tabbing_emoji_lookup "$opt_emoji" >/dev/null 2>&1; then
      _tabbing_emoji_lookup "$opt_emoji"
      return 1
    fi
    export TAB_EMOJI="$opt_emoji"
  fi

  if [[ $no_emoji -eq 1 ]]; then
    unset TAB_EMOJI
  fi

  # Positional args: bash arrays are 0-based
  if [[ ${#positional[@]} -ge 1 ]]; then
    export TAB_TITLE="${positional[0]}"
    if [[ ${#positional[@]} -ge 2 ]]; then
      export TAB_STATUS="${positional[1]}"
    else
      unset TAB_STATUS
    fi
  fi

  # Inline: ensure TAB_ID exists
  if [[ -z "${TAB_ID:-}" ]]; then
    if [[ -r /dev/urandom ]]; then
      TAB_ID="$(od -An -tx1 -N4 /dev/urandom 2>/dev/null | tr -d ' \n')"
    fi
    [[ -z "${TAB_ID:-}" ]] && TAB_ID="$(printf '%04x%04x' $$ "$(date +%s)" | cut -c1-8)"
    export TAB_ID
  fi

  # Render immediately (from render.sh)
  if [[ -n "$TAB_TITLE" ]]; then
    _tabbing_render
    _tabbing_apply_urgency_color
  fi

  # Commit side effects (history + display + session save)
  _tabbing_commit "title"

  # Handle recording flags (on-demand source)
  if [[ $has_record -eq 1 || $has_continue -eq 1 || $has_stop_recording -eq 1 ]]; then
    if [ -z "${_TABBING_FULL_LIBS:-}" ]; then
      . "$_tabbing_root/lib/core.sh"
      . "$_tabbing_root/lib/history.sh"
      . "$_tabbing_root/lib/recording.sh"
      . "$_tabbing_root/lib/session.sh"
    fi
    _tabbing_handle_recording "$has_record" "$has_continue" "$has_stop_recording"
    _tabbing_session_save
  fi
}

# ---------------------------------------------------------------------------
# tabbing-status [flags] "text"
# ---------------------------------------------------------------------------
tabbing-status() {
  local opt_urgency="" opt_emoji=""
  local has_urgency=0 has_emoji=0 no_emoji=0
  local has_record=0 has_continue=0 has_stop_recording=0
  local positional=()

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --urgency=*) opt_urgency="${1#--urgency=}"; has_urgency=1; shift ;;
      --urgency)   opt_urgency="$2";              has_urgency=1; shift 2 ;;
      --pri=*)     opt_urgency="${1#--pri=}";      has_urgency=1; shift ;;
      --pri)       opt_urgency="$2";               has_urgency=1; shift 2 ;;
      -p)          opt_urgency="$2";               has_urgency=1; shift 2 ;;
      -pri[0-5])   opt_urgency="${1#-pri}";        has_urgency=1; shift ;;

      --emoji=*)   opt_emoji="${1#--emoji=}"; has_emoji=1; shift ;;
      --emoji)     opt_emoji="$2";            has_emoji=1; shift 2 ;;
      -e)          opt_emoji="$2";            has_emoji=1; shift 2 ;;
      --no-emoji)  no_emoji=1; shift ;;

      --record)         has_record=1; shift ;;
      --continue)       has_continue=1; shift ;;
      --stop-recording) has_stop_recording=1; shift ;;

      # -EMOJI shorthand
      -[a-z]*)
        local maybe="${1#-}"
        if [[ "$maybe" =~ ^pri[0-5]$ ]]; then
          opt_urgency="${maybe#pri}"; has_urgency=1; shift
        elif _tabbing_is_known_emoji "$maybe"; then
          opt_emoji="$maybe"; has_emoji=1; shift
        else
          positional+=("$1"); shift
        fi
        ;;

      *)           positional+=("$1"); shift ;;
    esac
  done

  if [[ -z "$TAB_TITLE" ]]; then
    echo "tabbing-status: no TAB_TITLE set — call tabbing-on first" >&2
    return 1
  fi

  if [[ $has_urgency -eq 1 ]]; then
    if [[ ! "$opt_urgency" =~ ^[0-5]$ ]]; then
      echo "tabbing: urgency must be 0-5 (0=critical, 5=nominal)" >&2
      return 1
    fi
    export TAB_URGENCY="$opt_urgency"
  fi

  if [[ $has_emoji -eq 1 ]]; then
    if ! _tabbing_emoji_lookup "$opt_emoji" >/dev/null 2>&1; then
      _tabbing_emoji_lookup "$opt_emoji"
      return 1
    fi
    export TAB_EMOJI="$opt_emoji"
  fi

  if [[ $no_emoji -eq 1 ]]; then
    unset TAB_EMOJI
  fi

  # Bash: 0-based arrays
  if [[ ${#positional[@]} -ge 1 ]]; then
    export TAB_STATUS="${positional[0]}"
  fi

  _tabbing_render
  _tabbing_apply_urgency_color

  # Commit side effects
  _tabbing_commit "status"

  # Handle recording (on-demand source)
  if [[ $has_record -eq 1 || $has_continue -eq 1 || $has_stop_recording -eq 1 ]]; then
    if [ -z "${_TABBING_FULL_LIBS:-}" ]; then
      . "$_tabbing_root/lib/core.sh"
      . "$_tabbing_root/lib/history.sh"
      . "$_tabbing_root/lib/recording.sh"
      . "$_tabbing_root/lib/session.sh"
    fi
    _tabbing_handle_recording "$has_record" "$has_continue" "$has_stop_recording"
    _tabbing_session_save
  fi
}

# ---------------------------------------------------------------------------
# tabbing-todo — delegates to bin/ script
# Interactive switch handled inline (needs TTY + env var setting)
# ---------------------------------------------------------------------------
tabbing-todo() {
  local _root="${TABBING_ROOT:-$_tabbing_root}"

  # Quick check: --switch/-n needs inline handling for env var setting
  local has_switch=0
  local has_record=0 has_continue=0 has_stop_recording=0
  local orig_args=("$@")

  for arg in "$@"; do
    case "$arg" in
      -n|--switch|--pick) has_switch=1 ;;
      --record)           has_record=1 ;;
      --continue)         has_continue=1 ;;
      --stop-recording)   has_stop_recording=1 ;;
    esac
  done

  if [[ $has_switch -eq 1 ]]; then
    if [[ -z "$TAB_TITLE" ]]; then
      echo "tabbing-todo: no TAB_TITLE set — call tabbing-on first" >&2
      return 1
    fi

    # Get pending list from bin/ script
    local pending
    pending="$("$_root/bin/tabbing-todo" --list-pending)"
    if [[ -z "$pending" ]]; then
      echo "tabbing: no pending todos to switch to" >&2
      return 1
    fi

    echo "Pending todos:"
    local ids=() titles=()
    while IFS=' ' read -r id title; do
      ids+=("$id")
      titles+=("$title")
      printf '  %s) %s\n' "$id" "$title"
    done <<< "$pending"

    printf '\nSwitch to todo #: '
    local choice
    read -r choice

    local valid=0
    for id in "${ids[@]}"; do
      if [[ "$choice" == "$id" ]]; then
        valid=1; break
      fi
    done

    if [[ $valid -eq 0 ]]; then
      echo "tabbing: invalid choice" >&2
      return 1
    fi

    # Get export statements from bin/ script and eval them
    local _exports
    _exports="$("$_root/bin/tabbing-todo" --export-switch "$choice")"
    if [[ -n "$_exports" ]]; then
      eval "$_exports"
      _tabbing_render
      _tabbing_apply_urgency_color
    fi

    # Handle recording if requested
    if [[ $has_record -eq 1 || $has_continue -eq 1 || $has_stop_recording -eq 1 ]]; then
      if [ -z "${_TABBING_FULL_LIBS:-}" ]; then
        . "$_tabbing_root/lib/core.sh"
        . "$_tabbing_root/lib/history.sh"
        . "$_tabbing_root/lib/recording.sh"
        . "$_tabbing_root/lib/session.sh"
      fi
      _tabbing_handle_recording "$has_record" "$has_continue" "$has_stop_recording"
      _tabbing_session_save
    fi
    return
  fi

  # All other todo operations delegate entirely to bin/ script
  "$_root/bin/tabbing-todo" "${orig_args[@]}"
}

# ---------------------------------------------------------------------------
# Read-only commands — pure delegation to bin/ scripts
# ---------------------------------------------------------------------------
tabbing-report() {
  "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-report" "$@"
}

tabbing-history() {
  "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-history" "$@"
}

tabbing-recordings() {
  "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-recordings" "$@"
}

tabbing-clear() {
  "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-clear" "$@"
}

tabbing-info() {
  "${TABBING_ROOT:-$_tabbing_root}/bin/tabbing-info" "$@"
}

# ---------------------------------------------------------------------------
# Prompt hook: tab title re-render + optional inline prompt prefix
# Auto-registered — lightweight no-op when tabbing is not active.
# Set TABBING_PROMPT=1 to prepend [indicator Title: Status] to your prompt.
# ---------------------------------------------------------------------------
_tabbing_precmd() {
  if [[ -n "${TAB_TITLE:-}" ]]; then
    _tabbing_render
  fi

  # TODO: TABBING_PROMPT — inline prompt prefix is disabled pending fix
  # for escape sequence / cursor corruption on Ghostty and Kitty.
}

# Register hook — append so we run LAST,
# after Ghostty/Kitty shell integration hooks that reset the title.
case "${PROMPT_COMMAND:-}" in
  *_tabbing_precmd*) ;;
  *) PROMPT_COMMAND="${PROMPT_COMMAND:+$PROMPT_COMMAND;}_tabbing_precmd" ;;
esac
