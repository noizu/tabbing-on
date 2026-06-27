// theme_data.rs — Rust mirror of shell-impl/lib/theme-data.sh
//
// A theme is ONE newline-delimited blob of 383 lines; the 1-based line number is
// the lookup key. See theme-data.sh for the authoritative line map. This module
// mirrors that shell behavior exactly so the `tabbing-theme` CLI can get/set/
// generate the format identically.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::color;

pub const THEME_DATA_LINES: usize = 383;

// --- channel hex (2 digits) -> cube level index 0..5 -------------------------
fn cube_level(ch: &str) -> Option<u16> {
    match ch.to_ascii_lowercase().as_str() {
        "00" => Some(0),
        "5f" => Some(1),
        "87" => Some(2),
        "af" => Some(3),
        "d7" => Some(4),
        "ff" => Some(5),
        _ => None,
    }
}

/// cube hex RRGGBB -> palette slot 16..231 (None if not a canonical cube color)
pub fn cube_slot(hex: &str) -> Option<u16> {
    let h = hex.strip_prefix('#').unwrap_or(hex);
    if h.len() != 6 {
        return None;
    }
    let r = cube_level(&h[0..2])?;
    let g = cube_level(&h[2..4])?;
    let b = cube_level(&h[4..6])?;
    Some(16 + 36 * r + 6 * g + b)
}

/// logical handle -> 1-based line number
pub fn handle_to_line(handle: &str) -> Option<usize> {
    match handle {
        "foreground" | "fg" => return Some(110),
        "background" | "bg" => return Some(111),
        "cursor" => return Some(112),
        _ => {}
    }

    if let Some(n) = handle.strip_prefix("slot-") {
        let n: usize = n.parse().ok()?;
        return Some(128 + n);
    }
    if let Some(n) = handle.strip_prefix("grayscale-") {
        let n: usize = n.parse().ok()?;
        return Some(360 + n);
    }

    // cube handles
    let cube_hex = handle
        .strip_prefix("THEME_CUBE_COLOR_")
        .or_else(|| handle.strip_prefix("cube-"));
    if let Some(hx) = cube_hex {
        let slot = cube_slot(hx)?;
        return Some(128 + slot as usize);
    }

    // bg-<name>: background named color = fg SGR code + 10
    if let Some(name) = handle.strip_prefix("bg-") {
        let fg = color::color_code_or_raw(name)?;
        let fg: u16 = fg.parse().ok()?;
        return match fg {
            30..=37 | 90..=97 => Some((fg + 10) as usize),
            _ => None,
        };
    }

    // bare integer -> raw SGR code / line number
    if !handle.is_empty() && handle.chars().all(|c| c.is_ascii_digit()) {
        return handle.parse().ok();
    }

    // named foreground color or attribute -> its SGR code is the line
    color::color_code_or_raw(handle).and_then(|c| c.parse().ok())
}

/// split a blob into its lines (1-based access by callers)
fn blob_lines(blob: &str) -> Vec<String> {
    if blob.is_empty() {
        return Vec::new();
    }
    blob.split('\n').map(|s| s.to_string()).collect()
}

/// pad a line vector to THEME_DATA_LINES (idempotent)
fn pad_lines(mut lines: Vec<String>) -> Vec<String> {
    if lines.len() < THEME_DATA_LINES {
        lines.resize(THEME_DATA_LINES, String::new());
    }
    lines
}

/// get the value at `handle` (empty -> None)
pub fn get(blob: &str, handle: &str) -> Option<String> {
    let line = handle_to_line(handle)?;
    if line == 0 {
        return None;
    }
    let lines = blob_lines(blob);
    let v = lines.get(line - 1)?;
    if v.is_empty() {
        None
    } else {
        Some(v.clone())
    }
}

/// resolve a value: #RRGGBB / on / off / empty pass through; else named ANSI ->
/// hex, then X11 db -> hex, else leave untouched.
pub fn resolve_value(v: &str) -> String {
    // #RRGGBB passthrough
    if let Some(h) = v.strip_prefix('#') {
        if h.len() == 6 && h.chars().all(|c| c.is_ascii_hexdigit()) {
            return v.to_string();
        }
    }
    if v == "on" || v == "off" || v.is_empty() {
        return v.to_string();
    }
    if let Some(hex) = color::color_to_hex(v) {
        return hex.to_string();
    }
    if let Some(hex) = x11_to_hex(v) {
        return hex;
    }
    v.to_string()
}

/// set the value at `handle`, padding to 383 lines, returning the new blob.
pub fn set(blob: &str, handle: &str, value: &str) -> String {
    let line = match handle_to_line(handle) {
        Some(l) if l >= 1 => l,
        _ => return blob.to_string(),
    };
    let resolved = resolve_value(value);
    let mut lines = pad_lines(blob_lines(blob));
    if line <= lines.len() {
        lines[line - 1] = resolved;
    }
    lines.join("\n")
}

// --- X11 color database ------------------------------------------------------

fn x11_data_path() -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(root) = std::env::var("TABBING_ROOT") {
        if !root.is_empty() {
            candidates.push(PathBuf::from(root).join("data/colors.txt"));
        }
    }
    let xdg = std::env::var("XDG_DATA_HOME").ok().filter(|s| !s.is_empty());
    let base = match xdg {
        Some(x) => PathBuf::from(x),
        None => {
            let home = std::env::var("HOME").unwrap_or_default();
            PathBuf::from(home).join(".local/share")
        }
    };
    candidates.push(base.join("tabbing-on/data/colors.txt"));
    // relative to the crate (in-repo layout): ../shell-impl/data/colors.txt
    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../shell-impl/data/colors.txt"),
    );

    candidates.into_iter().find(|p| p.is_file())
}

fn x11_map() -> &'static HashMap<String, String> {
    static MAP: OnceLock<HashMap<String, String>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        if let Some(path) = x11_data_path() {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                for raw in contents.lines() {
                    let line = raw.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    if let Some((name, hex)) = line.split_once('|') {
                        m.insert(name.to_string(), hex.to_string());
                    }
                }
            }
        }
        m
    })
}

/// normalize an X11/Tk color name and look it up in colors.txt
pub fn x11_to_hex(name: &str) -> Option<String> {
    let normalized = normalize_x11_name(name);
    x11_map().get(&normalized).cloned()
}

fn normalize_x11_name(name: &str) -> String {
    // lowercase, runs of whitespace -> single '-'
    let lower = name.to_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut in_ws = false;
    for c in lower.chars() {
        if c.is_whitespace() {
            if !in_ws {
                out.push('-');
                in_ws = true;
            }
        } else {
            out.push(c);
            in_ws = false;
        }
    }
    out
}

// --- ramp generation ---------------------------------------------------------

fn hex_channel(hex: &str, start: usize) -> u16 {
    u16::from_str_radix(&hex[start..start + 2], 16).unwrap_or(0)
}

/// integer RGB lerp between two #RRGGBB at `steps` evenly-spaced points,
/// inclusive of both endpoints. Round-half-up; uppercase #RRGGBB.
pub fn gen_ramp(from: &str, to: &str, steps: usize) -> Vec<String> {
    // Resolve named / X11 colors to hex first; otherwise a 6-char name like
    // "purple" passes the length check and hex_channel's unwrap_or(0) silently
    // zeroes every channel, yielding an all-#000000 ramp.
    let from = resolve_value(from);
    let to = resolve_value(to);
    let f = from.strip_prefix('#').unwrap_or(&from);
    let t = to.strip_prefix('#').unwrap_or(&to);
    let is_hex6 = |s: &str| s.len() == 6 && s.bytes().all(|b| b.is_ascii_hexdigit());
    if !is_hex6(f) || !is_hex6(t) {
        return Vec::new();
    }
    let (ar, ag, ab) = (hex_channel(f, 0), hex_channel(f, 2), hex_channel(f, 4));
    let (br, bg, bb) = (hex_channel(t, 0), hex_channel(t, 2), hex_channel(t, 4));

    let n = if steps < 2 { 2 } else { steps };
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let frac = i as f64 / (n - 1) as f64;
        let lerp = |a: u16, b: u16| -> u16 {
            (a as f64 + (b as f64 - a as f64) * frac + 0.5) as u16
        };
        out.push(format!(
            "#{:02X}{:02X}{:02X}",
            lerp(ar, br),
            lerp(ag, bg),
            lerp(ab, bb)
        ));
    }
    out
}

// --- standard xterm-256 palette ---------------------------------------------

/// fill blob lines 144..359 with the canonical 6x6x6 cube and 360..383 with the
/// 24-step grayscale ramp, leaving all other lines untouched.
pub fn gen_standard_palette(blob: &str) -> String {
    const LV: [u16; 6] = [0, 95, 135, 175, 215, 255];
    let mut lines = pad_lines(blob_lines(blob));
    for n in 1..=THEME_DATA_LINES {
        if (144..=359).contains(&n) {
            let idx = (n - 128) - 16; // cube slot 16..231 offset
            let r = idx / 36;
            let g = (idx % 36) / 6;
            let b = idx % 6;
            lines[n - 1] = format!("#{:02X}{:02X}{:02X}", LV[r], LV[g], LV[b]);
        } else if (360..=383).contains(&n) {
            let g = ((n - 360) * 10 + 8) as u16;
            lines[n - 1] = format!("#{:02X}{:02X}{:02X}", g, g, g);
        }
    }
    lines.join("\n")
}

/// a blob of 383 blank lines
pub fn empty_blob() -> String {
    vec![""; THEME_DATA_LINES].join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_slot() {
        // 00/00/00 -> 16
        assert_eq!(cube_slot("#000000"), Some(16));
        // ff/ff/ff -> 16 + 36*5 + 6*5 + 5 = 231
        assert_eq!(cube_slot("#FFFFFF"), Some(231));
        // 5f/87/af -> 16 + 36*1 + 6*2 + 3 = 67
        assert_eq!(cube_slot("#5f87af"), Some(67));
        // non-cube channel
        assert_eq!(cube_slot("#123456"), None);
        assert_eq!(cube_slot("#fff"), None);
    }

    #[test]
    fn test_handle_to_line() {
        assert_eq!(handle_to_line("foreground"), Some(110));
        assert_eq!(handle_to_line("fg"), Some(110));
        assert_eq!(handle_to_line("background"), Some(111));
        assert_eq!(handle_to_line("bg"), Some(111));
        assert_eq!(handle_to_line("cursor"), Some(112));
        assert_eq!(handle_to_line("slot-0"), Some(128));
        assert_eq!(handle_to_line("slot-255"), Some(383));
        assert_eq!(handle_to_line("grayscale-0"), Some(360));
        assert_eq!(handle_to_line("grayscale-23"), Some(383));
        assert_eq!(handle_to_line("cube-000000"), Some(144)); // 128+16
        assert_eq!(handle_to_line("cube-FFFFFF"), Some(359)); // 128+231
        assert_eq!(handle_to_line("THEME_CUBE_COLOR_000000"), Some(144));
        // named foreground color -> SGR code
        assert_eq!(handle_to_line("bright-red"), Some(91));
        assert_eq!(handle_to_line("red"), Some(31));
        assert_eq!(handle_to_line("underline"), Some(4));
        // bg-<name> -> fg + 10
        assert_eq!(handle_to_line("bg-blue"), Some(44));
        assert_eq!(handle_to_line("bg-bright-blue"), Some(104));
        // bare integer
        assert_eq!(handle_to_line("196"), Some(196));
    }

    #[test]
    fn test_get_set() {
        let blob = empty_blob();
        assert_eq!(get(&blob, "fg"), None);
        let blob = set(&blob, "fg", "#1E1E2E");
        assert_eq!(get(&blob, "fg"), Some("#1E1E2E".to_string()));
        // 383 lines preserved
        assert_eq!(blob.split('\n').count(), 383);
        // set via named color resolves to hex
        let blob = set(&blob, "background", "dracula");
        assert_eq!(get(&blob, "background"), Some("#282A36".to_string()));
        // attribute toggle
        let blob = set(&blob, "underline", "on");
        assert_eq!(get(&blob, "underline"), Some("on".to_string()));
    }

    #[test]
    fn test_resolve_value() {
        assert_eq!(resolve_value("#ABCDEF"), "#ABCDEF");
        assert_eq!(resolve_value("on"), "on");
        assert_eq!(resolve_value("off"), "off");
        assert_eq!(resolve_value(""), "");
        assert_eq!(resolve_value("red"), "#CC3333");
        // X11 lookup (data file present in repo layout)
        assert_eq!(resolve_value("dodger blue"), "#1E90FF");
        // shell normalize only lowercases + collapses whitespace; it does NOT
        // split CamelCase, so "DodgerBlue" -> "dodgerblue" (no match) passes through
        assert_eq!(resolve_value("DodgerBlue"), "DodgerBlue");
        // unknown -> untouched
        assert_eq!(resolve_value("not-a-color-xyz"), "not-a-color-xyz");
    }

    #[test]
    fn test_gen_ramp() {
        let r = gen_ramp("#5A0000", "#FF6E6E", 6);
        assert_eq!(r.len(), 6);
        assert_eq!(r[0], "#5A0000");
        assert_eq!(r[5], "#FF6E6E");
    }

    #[test]
    fn test_gen_ramp_named_endpoints() {
        // named colors must resolve to hex, not be parsed as raw hex digits
        let r = gen_ramp("red", "bright-red", 5);
        assert_eq!(r.len(), 5);
        assert_eq!(r[0], "#CC3333"); // red
        assert_eq!(r[4], "#FF6666"); // bright-red
        assert_eq!(r[0], resolve_value("red"));
        assert_eq!(r[4], resolve_value("bright-red"));
        // mixed named/hex endpoints also resolve
        let b = gen_ramp("red", "blue", 3);
        assert_eq!(b[0], "#CC3333");
        assert_eq!(b[2], "#3333CC");
        // 6-char names (the original #000000 bug: "purple"/"yellow") resolve too
        let p = gen_ramp("purple", "yellow", 4);
        assert_eq!(p.len(), 4);
        assert_ne!(p[0], "#000000");
        // a genuine 6-char non-color string must yield empty, never silent black
        assert!(gen_ramp("zzzzzz", "yellow", 4).is_empty());
        assert!(gen_ramp("notacolor", "white", 3).is_empty());
    }

    #[test]
    fn test_gen_standard_palette() {
        let blob = gen_standard_palette(&empty_blob());
        // line 144 = cube slot 16 = #000000
        assert_eq!(get(&blob, "144"), Some("#000000".to_string()));
        // line 359 = cube slot 231 = #FFFFFF
        assert_eq!(get(&blob, "359"), Some("#FFFFFF".to_string()));
        // grayscale line 360 = (0*10+8)=8 -> #080808
        assert_eq!(get(&blob, "360"), Some("#080808".to_string()));
        // grayscale line 383 = (23*10+8)=238 -> #EEEEEE
        assert_eq!(get(&blob, "383"), Some("#EEEEEE".to_string()));
        assert_eq!(blob.split('\n').count(), 383);
    }

    #[test]
    fn test_x11_to_hex() {
        assert_eq!(x11_to_hex("dodger-blue"), Some("#1E90FF".to_string()));
        assert_eq!(x11_to_hex("Dodger Blue"), Some("#1E90FF".to_string()));
    }
}
