use std::env;
use std::path::PathBuf;

/// Resolve the TABBING_ROOT directory (where lib/ and shell/ live).
fn resolve_tabbing_root() -> PathBuf {
    // Check if running from source tree: exe is in bin/, sibling to lib/ and shell/
    if let Ok(exe) = env::current_exe() {
        if let Some(parent) = exe.parent() {
            let candidate = parent.join("..").canonicalize().unwrap_or_default();
            if candidate.join("lib/render.sh").is_file() {
                return candidate;
            }
        }
    }

    // Installed location: ~/.local/share/tabbing-on
    let xdg_data = env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| format!("{}/.local/share", env::var("HOME").unwrap_or_default()));
    PathBuf::from(xdg_data).join("tabbing-on")
}

fn emit_purge() {
    // A fresh interactive shell must not inherit ANY appearance state from a
    // previous session — otherwise the prompt hook re-applies the last tab's
    // theme. TAB_THEME_DATA (the 383-line palette blob), TAB_BG, and the
    // _TABBING_WAS_ACTIVE / TAB_MARQUEE flags are just as load-bearing as
    // TAB_THEME here, so purge them too.
    println!("unset TAB_TITLE TAB_STATUS TAB_HIGHLIGHT TAB_TITLE_STYLE TAB_STATUS_STYLE TAB_URGENCY TAB_EMOJI TAB_THEME TAB_THEME_DATA TAB_BG TAB_MARQUEE");
    println!("unset TAB_TERMINAL TAB_SESSION TAB_ID TAB_COLOR_SUPPORT");
    println!("unset TABBING_TTY TABBING_DC_DAEMON_PID");
    println!("unset TABBING_PIPE TABBING_RUN_WITH_PIPE TABBING_DC_UUID DC_TAB_NS _TABBING_OWNER_PID _TABBING_WAS_ACTIVE");
    println!("unset CLAUDE_CODE_DISABLE_TERMINAL_TITLE");
}

fn emit_checks(shell: &str) {
    let rcfile = match shell {
        "bash" => ".bashrc",
        "zsh" => ".zshrc",
        _ => ".profile",
    };

    // The checks are shell code that gets eval'd by the user's shell.
    // We emit them as literal strings.
    println!(r#"# tabbing-on: startup config checks"#);
    println!(r#"if [ -z "${{TABBING_NO_WARN:-}}" ]; then"#);
    println!(r#"  _tabbing_init_warn() {{ printf '\033[31m[tabbing-on] %s\033[0m\n' "$1" >&2; }}"#);
    println!(r#"  _tabbing_init_notitle=0"#);
    println!(r#"  case "${{TERM:-}}" in"#);
    println!(r#"    xterm-kitty)"#);
    println!(r#"      _tabbing_init_conf="${{XDG_CONFIG_HOME:-$HOME/.config}}/kitty/kitty.conf""#);
    println!(r#"      if [ -f "$_tabbing_init_conf" ]; then"#);
    println!(
        r#"        if grep -v '^ *#' "$_tabbing_init_conf" | grep -q 'no-title' 2>/dev/null; then"#
    );
    println!(r#"          _tabbing_init_notitle=1"#);
    println!(r#"        fi"#);
    println!(r#"      fi"#);
    println!(r#"      if [ "$_tabbing_init_notitle" -eq 0 ]; then"#);
    println!(
        r#"        _tabbing_init_warn "Kitty shell_integration missing 'no-title' — tab titles will be overwritten""#
    );
    println!(
        r#"        _tabbing_init_warn "Fix: add 'shell_integration no-title' to kitty.conf, or run: tabbing-doctor""#
    );
    println!(
        r#"        _tabbing_init_warn "Suppress: export TABBING_NO_WARN=1 in {} before tabbing-init""#,
        rcfile
    );
    println!(r#"      fi"#);
    println!(r#"      ;;"#);
    println!(r#"    xterm-ghostty)"#);
    // GHOSTTY_SHELL_FEATURES is the ground truth: Ghostty stamps it at app
    // launch, so it reflects what the running instance actually enabled — a
    // config patched with no-title after launch is not in effect yet.
    println!(r#"      _tabbing_init_confpatched=0"#);
    println!(r#"      for _tabbing_init_conf in \"#);
    println!(r#"        "$HOME/Library/Application Support/com.mitchellh.ghostty/config" \"#);
    println!(r#"        "${{XDG_CONFIG_HOME:-$HOME/.config}}/ghostty/config"; do"#);
    println!(r#"        if [ -f "$_tabbing_init_conf" ]; then"#);
    println!(
        r#"          if grep -v '^ *#' "$_tabbing_init_conf" | grep -q 'no-title' 2>/dev/null; then"#
    );
    println!(r#"            _tabbing_init_confpatched=1"#);
    println!(r#"          fi"#);
    println!(r#"        fi"#);
    println!(r#"      done"#);
    println!(r#"      if [ -n "${{GHOSTTY_SHELL_FEATURES:-}}" ]; then"#);
    println!(r#"        case ",${{GHOSTTY_SHELL_FEATURES}}," in"#);
    println!(r#"          *,title,*|*,title:*) _tabbing_init_notitle=0 ;;"#);
    println!(r#"          *) _tabbing_init_notitle=1 ;;"#);
    println!(r#"        esac"#);
    println!(r#"      else"#);
    println!(r#"        _tabbing_init_notitle="$_tabbing_init_confpatched""#);
    println!(r#"      fi"#);
    println!(r#"      if [ "$_tabbing_init_notitle" -eq 0 ]; then"#);
    println!(r#"        if [ "$_tabbing_init_confpatched" -eq 1 ]; then"#);
    println!(
        r#"          _tabbing_init_warn "Ghostty title integration still ACTIVE in this session — config has no-title but was read at app launch""#
    );
    println!(
        r#"          _tabbing_init_warn "Fix: fully restart Ghostty (all windows) to apply shell-integration-features = no-title""#
    );
    println!(r#"        else"#);
    println!(
        r#"          _tabbing_init_warn "Ghostty shell-integration-features missing 'no-title' — tab titles will be overwritten""#
    );
    println!(
        r#"          _tabbing_init_warn "Fix: add 'shell-integration-features = no-title' to ghostty config, or run: tabbing-doctor""#
    );
    println!(r#"        fi"#);
    println!(
        r#"        _tabbing_init_warn "Suppress: export TABBING_NO_WARN=1 in {} before tabbing-init""#,
        rcfile
    );
    println!(r#"      fi"#);
    println!(r#"      unset _tabbing_init_confpatched"#);
    println!(r#"      ;;"#);
    println!(r#"  esac"#);
    println!(r#"  case "${{TERM:-}}" in"#);
    println!(r#"    xterm-kitty|xterm-ghostty)"#);
    println!(r#"      if [ -n "${{TERM_FOR_SSHD:-}}" ]; then"#);
    println!(r#"        if [ -n "${{TABBING_SSH_SHIM:-}}" ]; then"#);
    println!(r#"          :"#);
    println!(r#"        else"#);
    println!(
        r#"        if [ -f "$HOME/{rcfile}" ] && ! grep -v '^ *#' "$HOME/{rcfile}" | grep -qE '(ssh\s*\(\)|function ssh)' 2>/dev/null; then"#,
        rcfile = rcfile
    );
    println!(r#"          _tabbing_init_warn "No ssh wrapper — remote hosts will get TERM=$TERM""#);
    println!(
        r#"          _tabbing_init_warn "Fix: add ssh() {{ TERM=\"\$TERM_FOR_SSHD\" command ssh \"\$@\"; }} to {}""#,
        rcfile
    );
    println!(r#"        fi"#);
    println!(r#"        fi"#);
    println!(r#"      fi"#);
    println!(r#"      ;;"#);
    println!(r#"  esac"#);
    println!(r#"  unset _tabbing_init_conf _tabbing_init_notitle"#);
    println!(r#"  unset -f _tabbing_init_warn 2>/dev/null"#);
    println!(r#"fi"#);
}

fn emit_ssh_shim(shell: &str) {
    println!("# tabbing-on: ssh TERM shim");
    println!("_tabbing_ssh_shim_needed=0");
    println!("case \"${{TERM:-}}\" in");
    println!("  xterm-ghostty|xterm-kitty) _tabbing_ssh_shim_needed=1 ;;");
    println!("esac");
    println!("if [ -n \"${{TERM_FOR_SSHD:-}}\" ]; then");
    println!("  _tabbing_ssh_shim_needed=1");
    println!("fi");
    println!("if [ \"$_tabbing_ssh_shim_needed\" -eq 1 ]; then");
    println!("  if [ -z \"${{TERM_FOR_SSHD:-}}\" ]; then");
    println!("    case \"${{TERM:-}}\" in");
    println!("      xterm-ghostty|xterm-kitty) export TERM_FOR_SSHD=\"xterm-256color\" ;;");
    println!("      *) export TERM_FOR_SSHD=\"${{TERM:-xterm-256color}}\" ;;");
    println!("    esac");
    println!("  fi");
    match shell {
        "zsh" => println!("  if ! (( $+functions[ssh] || $+aliases[ssh] )); then"),
        _ => println!("  if ! declare -F ssh >/dev/null 2>&1 && ! alias ssh >/dev/null 2>&1; then"),
    }
    println!("    ssh() {{ TERM=\"$TERM_FOR_SSHD\" command ssh \"$@\"; }}");
    println!("    export TABBING_SSH_SHIM=1");
    println!("  fi");
    println!("fi");
    println!("unset _tabbing_ssh_shim_needed");
}

fn emit_dc_init(tabbing_root: &str) {
    // UUID is generated at init time by the shell adapter (tabbing.zsh/tabbing.bash)
    // when TABBING_ON_DC_MODE=1. We just need to seed dc with current state.
    println!(r#"# tabbing-on: direnv-config mode — init dc tab state"#);
    println!(r#"source "{}/lib/dc.sh""#, tabbing_root);
    println!(r#"_tabbing_dc_init"#);
}

fn print_help(tabbing_root: &str) {
    println!("tabbing-init \u{2014} Initialize tabbing-on for your shell");
    println!();
    println!("Usage:");
    println!("  eval \"$(tabbing-init bash)\"   # add to .bashrc");
    println!("  eval \"$(tabbing-init zsh)\"    # add to .zshrc");
    println!();
    println!("Alternatively, add the source line directly:");
    println!(
        "  source \"{}/shell/tabbing.bash\"   # .bashrc",
        tabbing_root
    );
    println!(
        "  source \"{}/shell/tabbing.zsh\"    # .zshrc",
        tabbing_root
    );
    println!();
    println!("Available commands after init:");
    println!("  tabbing-on         Set/display tab title, status, highlight, urgency, emoji");
    println!("  tabbing-off        Deactivate tabbing and restore terminal defaults");
    println!("  tabbing-status     Update status text and urgency/emoji");
    println!("  tabbing-todo       Manage per-tab todo items");
    println!("  tabbing-report     View time-in-state reports and charts");
    println!("  tabbing-history    Search/browse tab history");
    println!("  tabbing-recordings List/manage terminal recordings");
}

pub fn run_ssh_shim(args: &[String]) {
    let shell = args.first().map(|s| s.as_str()).unwrap_or("sh");
    match shell {
        "bash" | "zsh" => emit_ssh_shim(shell),
        "--help" | "-h" | "help" => {
            println!("tabbing-ssh-shim — emit shell code for ssh TERM override");
            println!();
            println!("Usage:");
            println!("  eval \"$(tabbing-ssh-shim bash)\"");
            println!("  eval \"$(tabbing-ssh-shim zsh)\"");
        }
        _ => {
            eprintln!("tabbing-ssh-shim: specify shell: bash or zsh");
            std::process::exit(1);
        }
    }
}

pub fn run_init(args: &[String]) {
    let tabbing_root = resolve_tabbing_root();
    let root_str = tabbing_root.to_string_lossy();

    // Parse flags
    let mut dc_mode = false;
    let mut positional: Vec<&str> = Vec::new();

    for arg in args {
        match arg.as_str() {
            "--direnv-config-mode" => dc_mode = true,
            _ => positional.push(arg),
        }
    }

    let shell = positional.first().map(|s| *s).unwrap_or("");

    match shell {
        "bash" => {
            emit_purge();
            println!("export TABBING_ROOT=\"{}\"", root_str);
            if dc_mode {
                println!("export TABBING_ON_DC_MODE=1");
            }
            println!("source \"{}/shell/tabbing.bash\"", root_str);
            emit_checks("bash");
            if dc_mode {
                emit_dc_init(&root_str);
            }
        }
        "zsh" => {
            emit_purge();
            println!("export TABBING_ROOT=\"{}\"", root_str);
            if dc_mode {
                println!("export TABBING_ON_DC_MODE=1");
            }
            println!("source \"{}/shell/tabbing.zsh\"", root_str);
            emit_checks("zsh");
            if dc_mode {
                emit_dc_init(&root_str);
            }
        }
        "bat" | "cmd" | "windows" => {
            println!("@echo off");
            println!("REM tabbing-on Windows support is experimental");
            println!("doskey tabbing-on=\"{}\\shell\\tabbing.bat\" $*", root_str);
        }
        "--help" | "-h" | "help" => {
            print_help(&root_str);
        }
        _ => {
            eprintln!("tabbing-init: specify your shell: bash or zsh");
            eprintln!();
            eprintln!("Usage:");
            eprintln!("  eval \"$(tabbing-init bash)\"   # add to .bashrc");
            eprintln!("  eval \"$(tabbing-init zsh)\"    # add to .zshrc");
            eprintln!();
            eprintln!("Run \"tabbing-init --help\" for more info.");
            std::process::exit(1);
        }
    }
}
