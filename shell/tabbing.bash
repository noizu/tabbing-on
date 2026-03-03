#!/bin/bash
# shell/tabbing.bash — Bash-specific tabbing-on command definitions
#
# Sources POSIX-compatible libs, then defines bash functions.
# Key difference from zsh: arrays are 0-based.
#
# Init: eval "$(tabbing-init bash)" in your .bashrc

# ---------------------------------------------------------------------------
# Locate and source shared libraries
# ---------------------------------------------------------------------------
_tabbing_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

source "${_tabbing_root}/lib/core.sh"
source "${_tabbing_root}/lib/terminal.sh"
source "${_tabbing_root}/lib/history.sh"
source "${_tabbing_root}/lib/recording.sh"
source "${_tabbing_root}/lib/todo.sh"

# Detect terminal once at init
_tabbing_detect_terminal

# Known color names for -color shorthand matching (bash array)
_tabbing_known_colors=(
  black red green yellow blue magenta cyan white
  bright-black bright-red bright-green bright-yellow
  bright-blue bright-magenta bright-cyan bright-white
  bold dim italic underline blink inverse strikethrough
)

# ---------------------------------------------------------------------------
# tabbing-on [flags in any position] [title] [status]
#
# Flags:
#   --highlight, -h, -COLOR        set title highlight color
#   --urgency, --pri, -p, -priN    set urgency 0-5
#   --emoji, -e, -EMOJI            set named emoji
#   --no-emoji                     clear emoji
#   --emoji-list                   show available emojis
#   --record                       start asciinema recording
#   --continue                     keep recording on status change
#   --stop-recording               stop recording
#   --terminal-info                show detected terminal info
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
      --emoji-list|--emojis) _tabbing_emoji_list; return ;;
      --help)        _tabbing_help; return ;;

      --record)         has_record=1; shift ;;
      --continue)       has_continue=1; shift ;;
      --stop-recording) has_stop_recording=1; shift ;;

      --terminal-info)  _tabbing_terminal_info; return ;;

      # -COLOR or -EMOJI shorthand (e.g. -blue, -rocket)
      -[a-z]*)
        local maybe="${1#-}"
        # Check for -priN first
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
      emojis|emoji-list) _tabbing_emoji_list; return ;;
      colors|color-list) _tabbing_color_list; return ;;
      help)              _tabbing_help; return ;;
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

  # Ensure TAB_ID exists for history tracking
  _tabbing_ensure_tab_id

  # Record history event
  if [[ -n "$TAB_TITLE" ]]; then
    _tabbing_record_event "title"
    _tabbing_render
    _tabbing_apply_urgency_color
  fi

  # Handle recording flags
  if [[ $has_record -eq 1 || $has_continue -eq 1 || $has_stop_recording -eq 1 ]]; then
    _tabbing_handle_recording "$has_record" "$has_continue" "$has_stop_recording"
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
  _tabbing_record_event "status"

  # Handle recording
  if [[ $has_record -eq 1 || $has_continue -eq 1 || $has_stop_recording -eq 1 ]]; then
    _tabbing_handle_recording "$has_record" "$has_continue" "$has_stop_recording"
  fi
}

# ---------------------------------------------------------------------------
# tabbing-todo — task/todo management per tab
# ---------------------------------------------------------------------------
tabbing-todo() {
  local opt_desc="" opt_emoji="" opt_urgency=""
  local has_desc=0 has_emoji=0 has_urgency=0
  local has_record=0 has_continue=0 has_stop_recording=0
  local do_switch=0 do_done=0 done_id=""
  local positional=()

  while [[ $# -gt 0 ]]; do
    case "$1" in
      -m|--message)     opt_desc="$2"; has_desc=1; shift 2 ;;
      -m=*|--message=*) opt_desc="${1#*=}"; has_desc=1; shift ;;

      --emoji=*)   opt_emoji="${1#--emoji=}"; has_emoji=1; shift ;;
      --emoji)     opt_emoji="$2"; has_emoji=1; shift 2 ;;
      -e)          opt_emoji="$2"; has_emoji=1; shift 2 ;;

      --urgency=*) opt_urgency="${1#--urgency=}"; has_urgency=1; shift ;;
      --urgency)   opt_urgency="$2"; has_urgency=1; shift 2 ;;
      --pri=*)     opt_urgency="${1#--pri=}"; has_urgency=1; shift ;;
      --pri)       opt_urgency="$2"; has_urgency=1; shift 2 ;;
      -p)          opt_urgency="$2"; has_urgency=1; shift 2 ;;
      -pri[0-5])   opt_urgency="${1#-pri}"; has_urgency=1; shift ;;

      -n|--switch|--pick) do_switch=1; shift ;;
      --done)
        do_done=1
        if [[ -n "${2:-}" && "$2" =~ ^[0-9]+$ ]]; then
          done_id="$2"; shift 2
        else
          shift
        fi
        ;;

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

      *) positional+=("$1"); shift ;;
    esac
  done

  if [[ -z "$TAB_TITLE" ]]; then
    echo "tabbing-todo: no TAB_TITLE set — call tabbing-on first" >&2
    return 1
  fi

  _tabbing_ensure_tab_id

  # Handle --done
  if [[ $do_done -eq 1 ]]; then
    _tabbing_todo_done "$done_id"
    return
  fi

  # Handle --switch / -n (interactive)
  if [[ $do_switch -eq 1 ]]; then
    local pending
    pending="$(_tabbing_todo_pending)"
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

    _tabbing_todo_switch "$choice"

    if [[ $has_record -eq 1 || $has_continue -eq 1 || $has_stop_recording -eq 1 ]]; then
      _tabbing_handle_recording "$has_record" "$has_continue" "$has_stop_recording"
    fi
    return
  fi

  # No positional args — list todos (bash: 0-based)
  if [[ ${#positional[@]} -eq 0 ]]; then
    _tabbing_todo_list
    return
  fi

  # Add a new todo (bash: 0-based)
  local title="${positional[0]}"
  _tabbing_todo_add "$title" "$opt_desc" "$opt_emoji" "$opt_urgency"

  if [[ $has_record -eq 1 || $has_continue -eq 1 || $has_stop_recording -eq 1 ]]; then
    _tabbing_handle_recording "$has_record" "$has_continue" "$has_stop_recording"
  fi
}

# ---------------------------------------------------------------------------
# tabbing-report — history reporting
# ---------------------------------------------------------------------------
tabbing-report() {
  local target_tab="${TAB_ID:-}"
  local do_mermaid=0 do_all=0 do_list=0
  local search_query=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --mermaid)     do_mermaid=1; shift ;;
      --tab)         target_tab="$2"; shift 2 ;;
      --tab=*)       target_tab="${1#--tab=}"; shift ;;
      --all)         do_all=1; shift ;;
      --list)        do_list=1; shift ;;
      --search)      search_query="$2"; shift 2 ;;
      --search=*)    search_query="${1#--search=}"; shift ;;
      *)             shift ;;
    esac
  done

  if [[ $do_list -eq 1 ]]; then
    _tabbing_history_list_tabs
    return
  fi

  if [[ -n "$search_query" ]]; then
    _tabbing_history_search "$search_query"
    return
  fi

  if [[ $do_all -eq 1 ]]; then
    if [[ $do_mermaid -eq 1 ]]; then
      local dir
      dir="$(_tabbing_history_dir)"
      for f in "$dir"/*.yaml; do
        [[ -f "$f" ]] || continue
        local tid
        tid="$(sed -n 's/^tab_id: "\(.*\)"/\1/p' "$f" | head -1)"
        _tabbing_report_mermaid "$tid"
        printf '\n'
      done
    else
      _tabbing_report_all
    fi
    return
  fi

  if [[ -z "$target_tab" ]]; then
    echo "tabbing-report: no TAB_ID set — call tabbing-on first" >&2
    return 1
  fi

  if [[ $do_mermaid -eq 1 ]]; then
    _tabbing_report_mermaid "$target_tab"
  else
    _tabbing_report "$target_tab"
  fi
}

# ---------------------------------------------------------------------------
# tabbing-history — search/browse history
# ---------------------------------------------------------------------------
tabbing-history() {
  if [[ $# -eq 0 ]]; then
    _tabbing_history_list_tabs
    return
  fi
  _tabbing_history_search "$*"
}

# ---------------------------------------------------------------------------
# tabbing-recordings — list/manage recordings
# ---------------------------------------------------------------------------
tabbing-recordings() {
  local target_tab="${TAB_ID:-}"

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --tab)    target_tab="$2"; shift 2 ;;
      --tab=*)  target_tab="${1#--tab=}"; shift ;;
      --to-gif)
        if [[ -n "${2:-}" ]]; then
          _tabbing_recording_to_gif "$2" "${3:-}"
          return
        else
          echo "tabbing-recordings: --to-gif requires a .cast file path" >&2
          return 1
        fi
        ;;
      *) shift ;;
    esac
  done

  _tabbing_recordings_list "$target_tab"
}

# ---------------------------------------------------------------------------
# tabbing-clear — clear history, todos, recordings
# ---------------------------------------------------------------------------
tabbing-clear() {
  local target="${1:-}"
  shift 2>/dev/null || true

  local tab_id="${TAB_ID:-}"
  local before="" after="" clear_all_tabs=0

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --tab)      tab_id="$2"; shift 2 ;;
      --tab=*)    tab_id="${1#--tab=}"; shift ;;
      --before)   before="$2"; shift 2 ;;
      --before=*) before="${1#--before=}"; shift ;;
      --after)    after="$2"; shift 2 ;;
      --after=*)  after="${1#--after=}"; shift ;;
      --all)      clear_all_tabs=1; shift ;;
      *)          shift ;;
    esac
  done

  _tabbing_ensure_tab_id

  case "$target" in
    history)
      if [[ $clear_all_tabs -eq 1 ]]; then
        local dir
        dir="$(_tabbing_history_dir)"
        rm -rf "$dir"
        printf 'tabbing: cleared history for all tabs\n'
      else
        _tabbing_clear_history "${tab_id:-$TAB_ID}" "$before" "$after"
      fi
      ;;
    todos)
      _tabbing_clear_todos "${tab_id:-$TAB_ID}"
      ;;
    recordings)
      _tabbing_clear_recordings "${tab_id:-$TAB_ID}"
      ;;
    all)
      _tabbing_clear_all "${tab_id:-$TAB_ID}"
      ;;
    everything)
      _tabbing_clear_everything
      ;;
    *)
      printf 'Usage: tabbing-clear history|todos|recordings|all|everything\n' >&2
      printf '\n' >&2
      printf '  history                 Clear history (current tab)\n' >&2
      printf '  history --all           Clear history (all tabs)\n' >&2
      printf '  history --before DATE   Clear entries before ISO date\n' >&2
      printf '  history --after DATE    Clear entries after ISO date\n' >&2
      printf '  todos                   Clear todos (current tab)\n' >&2
      printf '  recordings              Clear recordings (current tab)\n' >&2
      printf '  all                     Clear everything (current tab)\n' >&2
      printf '  everything              Clear everything (all tabs)\n' >&2
      return 1
      ;;
  esac
}

# ---------------------------------------------------------------------------
# tabbing-info — full info dump (state, paths, recordings, todos)
# ---------------------------------------------------------------------------
tabbing-info() {
  _tabbing_ensure_tab_id
  _tabbing_info "${1:-$TAB_ID}"
}

# ---------------------------------------------------------------------------
# Optional: auto-render via PROMPT_COMMAND
# ---------------------------------------------------------------------------
_tabbing_precmd() {
  if [[ -n "${TAB_TITLE:-}" ]]; then
    _tabbing_render
  fi
}
# Uncomment to enable:
# PROMPT_COMMAND="_tabbing_precmd;${PROMPT_COMMAND:-}"
