# Theme data format (`TAB_THEME_DATA`)

A theme is stored in **one variable** — `TAB_THEME_DATA` — as a newline-delimited
block of **383 lines** where the *line number is the lookup key*. No YAML, no
JSON, no `yq`: reading a value is `sed -n "${line}p"`. On disk the same blob is
saved verbatim as `~/.config/tabbing-on/themes/<name>.themedata`.

## Line map

| Lines     | Keyed by            | Holds                                   |
|-----------|---------------------|-----------------------------------------|
| 1–9       | SGR code            | attribute toggles (`bold`=1, `underline`=4, …) → `on`/blank |
| 30–37     | SGR code            | standard **foreground** colors → `#RRGGBB` |
| 40–47     | SGR code            | standard **background** colors → `#RRGGBB` |
| 50–65     | SGR code            | special attributes (`framed`=51, `encircled`=52, `overlined`=53, …) → `on`/blank |
| 90–97     | SGR code            | bright **foreground** → `#RRGGBB`       |
| 100–107   | SGR code            | bright **background** → `#RRGGBB`       |
| 110/111/112 | OSC number (mnemonic) | foreground / background / cursor → `#RRGGBB` |
| 128–383   | `128 + slot`        | xterm-256 palette: 0–15 → 128–143, cube 16–231 → 144–359, grayscale 232–255 → 360–383 |

SGR-indexed color lines (31/91/…) are the *text* colors and may differ from the
OSC-4 palette slots (128+). Mappings verified against the
[ANSI escape code spec](https://en.wikipedia.org/wiki/ANSI_escape_code):
cube = `16 + 36·r + 6·g + b`, levels `0,95,135,175,215,255`; grayscale = `10·n + 8`.

## Handles → line

| Handle | Line |
|---|---|
| `foreground` \| `fg` / `background` \| `bg` / `cursor` | 110 / 111 / 112 |
| `slot-<N>` (0–255) | `128 + N` |
| `grayscale-<N>` (0–23) | `360 + N` |
| `THEME_CUBE_COLOR_RRGGBB` \| `cube-RRGGBB` | `128 + cube_slot(RRGGBB)` |
| `bg-<name>` | named fg SGR + 10 |
| named ANSI color / attribute (`bright-red`, `underline`) | its SGR code |
| bare integer | that line |

`THEME_CUBE_COLOR_FF0000` → r5 g0 b0 → slot 196 → line 324. Channels map
`00 5f 87 af d7 ff → 0..5`.

## Values

A value may be `#RRGGBB`, a named ANSI color, an **X11 symbolic name**
(`dodger-blue`, `midnight blue` → resolved via `data/colors.txt`, 657 names), or
`on`/`off` for attribute lines. `_tabbing_resolve_value` normalizes on `set`.

## Shell API (`lib/theme-data.sh`, pure POSIX + sed/awk)

```sh
_tabbing_theme_get   <handle> [var]            # sed -n line
_tabbing_theme_set   <handle> <value> [var]    # rewrite line (resolves value)
_tabbing_theme_line  <handle>                  # handle → line number
_tabbing_x11_to_hex  <name>                    # X11 name → #RRGGBB
_tabbing_gen_ramp    <from#> <to#> <steps>     # integer RGB lerp, inclusive
_tabbing_gen_standard_palette [var]            # fill cube+grayscale (xterm canon)
_tabbing_theme_attr_codes                      # enabled SGR attrs → "4;53"
_tabbing_save_theme_data <name> / _tabbing_load_theme_data <name>
```

## Apply / persist path

`_tabbing_send_theme` (and built-in/user themes) populate the blob, then
`_tabbing_emit_theme_data` streams `OSC 4;N` per non-empty slot + `OSC 10/11/12`
for ui. The prompt hook `_tabbing_persist_theme` re-emits the blob every prompt
so Ghostty/Kitty palette resets get stomped back (opt out: `TABBING_THEME_PERSIST=0`).

## CLI

```sh
tabbing-theme get <handle> [name]
tabbing-theme set <handle> <value> [name]
tabbing-theme gen-standard [name]
tabbing-theme gen-ramp <from> <to> <steps>
```
