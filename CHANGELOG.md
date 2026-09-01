# Changelog — utilities/shell/tabbing-on

## [Unreleased]
- Fixed: `tabbing-init` purge now also clears `DC_TAB_NS`, `_TABBING_OWNER_PID`, `_TABBING_WAS_ACTIVE`, `TAB_THEME_DATA`, `TAB_BG`, `TAB_MARQUEE`, and style vars so inherited tab state cannot leak into a new shell
- Fixed: `use_tabbing_dc` reads `${DC_TAB_NS:-tab}` instead of hardcoded global `tab`
- Companion: `direnv-config` `dc-init` protects `TAB_*` from `tab.*: TAB_*` flatten clobber (see direnv-config CHANGELOG)

## [m6-arch-layout-docs] — 2026-07-16 — tag: `utilities-shell-tabbing-on/m6-arch-layout-docs`
Milestone summary: PROJ-ARCH/PROJ-LAYOUT docs (and per-tier summaries) rewritten to reflect the current rust + shell-impl split, with new per-directory layout docs for each subsystem.

### Added
- `docs/layout/rust.md`, `docs/layout/shell-impl.md` — per-subsystem layout references
### Changed
- `docs/PROJ-ARCH.md`, `docs/PROJ-ARCH.summary.md`, `docs/PROJ-LAYOUT.md`, `docs/PROJ-LAYOUT.summary.md` restructured to match current architecture

## [m5-theme-picker-and-doctor-hardening] — 2026-07-16 — tag: `utilities-shell-tabbing-on/m5-theme-picker-and-doctor-hardening`
Milestone summary: major overhaul of the interactive theme picker plus a hardening pass on init/doctor and shell entrypoints.

### Added
- `shell-impl/examples/themes/my-dark.theme` example theme
- `tabbing-doctor`, expanded `tabbing-init` diagnostics/setup checks
### Changed
- `theme_picker.rs` substantially rewritten (~1450 lines changed) — richer picker UX
- `render.sh`, `tabbing.bash`/`tabbing.zsh` updated for new picker + init/doctor flows
- `theme.rs`, `theme_data.rs`, `main.rs` adjusted to support picker changes
- `doctor.rs`, `init.rs` hardened with additional checks

## [m4-session-isolation-and-dc-hardening] — 2026-07-07 — tag: `utilities-shell-tabbing-on/m4-session-isolation-and-dc-hardening`
Milestone summary: fixed theme/tab state leaking across terminal sessions and hardened the shared `dc` (direnv-config) store against lost writes; includes preceding small patches (Makefile, todo.rs) from the same work stretch.

### Added
- Owner-PID hygiene block in `tabbing.zsh`/`tabbing.bash`: a new shell process evicts inherited `TAB_*` appearance and regenerates session identity
- `todo.rs` command support; Makefile convenience targets
### Changed
- `dc-init` precmd bridge now reads session-scoped `${DC_TAB_NS:-tab}` instead of a hardcoded global tab config; `tabbing-on` exports `DC_TAB_NS` in lockstep with `TABBING_DC_UUID` generation
- All tab `dc` writes unified on `--layer base` (previously `dc set` defaulted to `local`, shadowing yaml `--replace` saves so cleared keys stuck)
- `emit_purge` / `tabbing-off` now also unset `TAB_THEME_DATA`, `TAB_BG`, `TAB_MARQUEE`, `DC_TAB_NS`, `_TABBING_OWNER_PID`, `_TABBING_WAS_ACTIVE`
- `dc`: exclusive flock on `<store>/.lock` for all read-modify-write commands (set/yaml/unset/prune/purge/bump) — no more lost version bumps; unparseable layers now error instead of being silently overwritten
### Fixed
- BSD `date +%s%3N` prints a literal `3N`; validated numeric before writing `last_update`
### Removed
- ~150 lines of dead shell theme-data value functions duplicated by `rust/src/theme_data.rs` (zero shell callers)

## [m3-256-color-theme-data-store] — 2026-06-27 — tag: `utilities-shell-tabbing-on/m3-256-color-theme-data-store`
Milestone summary: replaced the ad-hoc theme variables with a single newline-delimited `TAB_THEME_DATA` blob (line number = lookup key), giving a zero-dependency 256-color theme system mirrored in both shell and Rust.

### Added
- `lib/theme-data.sh`: handle→line map, get/set, cube reverse-mapping, X11 name resolution, `gen_ramp`, `gen_standard_palette`, attribute toggles, `.themedata` save/load
- `data/colors.txt`: 657 X11/Tk color names → hex (verified against `rgb.txt`)
- `rust/src/theme_data.rs`: byte-for-byte mirror of the shell store; `theme.rs` `resolve_palette`; new `tabbing-theme get/set/gen-standard/gen-ramp` CLI
- `theme_picker.rs` `g` key: generate a full 256-color `<name>-256.themedata`
- `docs/theme-data-format.md`: line-map + API reference
### Changed
- `render.sh` apply path and per-prompt re-assert now read the theme-data blob; new `_tabbing_emit_theme_data` / `_tabbing_persist_theme` with Ghostty palette-reset defense (opt out via `TABBING_THEME_PERSIST=0`); attribute codes merged into title

## [m2-rust-engine-buildout] — 2026-06-26 — tag: `utilities-shell-tabbing-on/m2-rust-engine-buildout`
Milestone summary: stood up the Rust port alongside the existing shell implementation and landed a wide feature buildout — emoji, plan sub-commands, recording, todo/toggl, history, doctor, init — plus the first color/theme groundwork.

### Added
- `rust/src/color.rs`, `theme.rs`, `theme_picker.rs` initial implementations
- `rust/src/emoji.rs`, `doctor.rs`, `history.rs`, `recording.rs`, `todo.rs`, `toggl.rs`, `terminal.rs`, `plan/{app,audio,filesystem,llm,mod}.rs`
- `shell-impl/bin/tabbing-init`
### Changed
- `main.rs`, `state.rs`, `bridge.rs`, `daemon.rs`, `init.rs` expanded to route through the new Rust subsystems
- `shell-impl/lib/dc.sh`, `lib/render.sh`, `lib/session.sh`, `shell/tabbing.bash`, `shell/tabbing.zsh` updated for Rust-backed rendering
- README updated to document the Rust build

## [m1-shell-bootstrap] — 2026-06-14 — tag: `utilities-shell-tabbing-on/m1-shell-bootstrap`
Milestone summary: baseline Makefile/gitignore hardening for the shell-first tool, adding a cargo-optional install path (falls back to shell-only install when Rust isn't available) ahead of the Rust port.

### Added
- `.gitignore` entries for editor swap files, `.env`, `**/*.rs.bk`
### Changed
- `Makefile` `compile`/`test`/`install` targets now detect `cargo` and fall back to `install-shell` when absent
