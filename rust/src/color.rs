pub fn color_code(name: &str) -> Option<&'static str> {
    match name {
        "black" => Some("30"),
        "red" => Some("31"),
        "green" => Some("32"),
        "yellow" => Some("33"),
        "blue" => Some("34"),
        "magenta" => Some("35"),
        "cyan" => Some("36"),
        "white" => Some("37"),
        "bright-black" => Some("90"),
        "bright-red" => Some("91"),
        "bright-green" => Some("92"),
        "bright-yellow" => Some("93"),
        "bright-blue" => Some("94"),
        "bright-magenta" => Some("95"),
        "bright-cyan" => Some("96"),
        "bright-white" => Some("97"),
        "bold" => Some("1"),
        "dim" => Some("2"),
        "italic" => Some("3"),
        "underline" => Some("4"),
        "blink" => Some("5"),
        "inverse" => Some("7"),
        "strikethrough" => Some("9"),
        s if s.chars().all(|c| c.is_ascii_digit()) => None, // raw code passthrough handled by caller
        _ => None,
    }
}

pub fn color_code_or_raw(name: &str) -> Option<String> {
    if let Some(code) = color_code(name) {
        return Some(code.to_string());
    }
    if name.chars().all(|c| c.is_ascii_digit()) {
        return Some(name.to_string());
    }
    None
}

pub fn is_known_color(name: &str) -> bool {
    matches!(
        name,
        "black"
            | "red"
            | "green"
            | "yellow"
            | "blue"
            | "magenta"
            | "cyan"
            | "white"
            | "bright-black"
            | "bright-red"
            | "bright-green"
            | "bright-yellow"
            | "bright-blue"
            | "bright-magenta"
            | "bright-cyan"
            | "bright-white"
            | "bold"
            | "dim"
            | "italic"
            | "underline"
            | "blink"
            | "inverse"
            | "strikethrough"
    )
}

pub fn urgency_ansi(level: u8) -> &'static str {
    match level {
        0 => "91", // bright red
        1 => "31", // red
        2 => "33", // yellow
        3 => "93", // bright yellow
        4 => "32", // green
        5 => "90", // gray
        _ => "0",
    }
}

pub fn urgency_dot(level: u8) -> &'static str {
    match level {
        0 => "\u{1F534}", // red circle
        1 => "\u{1F7E0}", // orange circle
        2 => "\u{1F7E1}", // yellow circle
        3 => "\u{1F7E2}", // green circle
        4 => "\u{1F7E2}", // green circle
        5 => "\u{26AA}",  // white/gray circle
        _ => "",
    }
}

pub fn color_to_hex(name: &str) -> Option<&'static str> {
    match name {
        s if s.starts_with('#') && s.len() == 7 => return None, // passthrough handled by caller
        "black" => Some("#000000"),
        "red" => Some("#CC3333"),
        "green" => Some("#33CC33"),
        "yellow" => Some("#CCCC33"),
        "blue" => Some("#3333CC"),
        "magenta" => Some("#CC33CC"),
        "cyan" => Some("#33CCCC"),
        "white" => Some("#FFFFFF"),
        "bright-black" => Some("#666666"),
        "bright-red" => Some("#FF6666"),
        "bright-green" => Some("#66FF66"),
        "bright-yellow" => Some("#FFFF66"),
        "bright-blue" => Some("#6666FF"),
        "bright-magenta" => Some("#FF66FF"),
        "bright-cyan" => Some("#66FFFF"),
        "bright-white" => Some("#FFFFFF"),
        "dark" => Some("#1E1E1E"),
        "midnight" => Some("#0D1117"),
        "charcoal" => Some("#2B2B2B"),
        "slate" => Some("#2E3440"),
        "graphite" => Some("#3B3B3B"),
        "obsidian" => Some("#1A1A2E"),
        "onyx" => Some("#0F0F0F"),
        "nord" => Some("#2E3440"),
        "dracula" => Some("#282A36"),
        "monokai" => Some("#272822"),
        "solarized-dark" => Some("#002B36"),
        "solarized-light" => Some("#FDF6E3"),
        "gruvbox" => Some("#282828"),
        "gruvbox-light" => Some("#FBF1C7"),
        "tokyo-night" => Some("#1A1B26"),
        "catppuccin" => Some("#1E1E2E"),
        "one-dark" => Some("#282C34"),
        "rose-pine" => Some("#191724"),
        "kanagawa" => Some("#1F1F28"),
        "ocean" => Some("#0A1628"),
        "forest" => Some("#0B2B1A"),
        "sunset" => Some("#2C1810"),
        "wine" => Some("#2C0B0E"),
        "plum" => Some("#2D1B30"),
        "storm" => Some("#1B2838"),
        "ember" => Some("#3B1518"),
        "moss" => Some("#1A2E1A"),
        "sand" => Some("#3B3426"),
        "ice" => Some("#1A2B3C"),
        "danger" => Some("#3B0F0F"),
        "warning" => Some("#3B2E0F"),
        "safe" => Some("#0F3B1A"),
        "info" => Some("#0F1E3B"),
        "navy" => Some("#001F3F"),
        "olive" => Some("#3D3D00"),
        "teal" => Some("#003B3B"),
        "maroon" => Some("#3B0000"),
        "purple" => Some("#2D0A3B"),
        "orange" => Some("#3B2200"),
        "brown" => Some("#2B1A0A"),
        "pink" => Some("#3B0A2D"),
        "gray" | "grey" => Some("#333333"),
        "silver" => Some("#C0C0C0"),
        _ => None,
    }
}

pub fn resolve_hex(name: &str) -> Option<String> {
    if name.starts_with('#') && name.len() == 7 {
        return Some(name.to_string());
    }
    color_to_hex(name).map(|s| s.to_string())
}

pub fn urgency_tab_color(level: u8) -> Option<(u8, u8, u8)> {
    match level {
        0 => Some((200, 50, 50)),
        1 => Some((200, 100, 50)),
        2 => Some((200, 180, 50)),
        3 => Some((150, 200, 50)),
        4 => Some((50, 180, 50)),
        5 => Some((120, 120, 120)),
        _ => None,
    }
}

// ============================================================================
// HSV / RGB / hex conversions (used by the theme-picker color wheel)
// ============================================================================

/// Parse a `#RRGGBB` string into an RGB triple. Returns `None` on bad input.
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

/// Render an RGB triple as an uppercase `#RRGGBB` string.
pub fn rgb_to_hex((r, g, b): (u8, u8, u8)) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Convert HSV (h in degrees [0,360), s & v in [0,1]) to an RGB triple.
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h.rem_euclid(360.0);
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let to_u8 = |f: f32| (((f + m) * 255.0).round()).clamp(0.0, 255.0) as u8;
    (to_u8(r1), to_u8(g1), to_u8(b1))
}

/// Convert an RGB triple to HSV (h in degrees [0,360), s & v in [0,1]).
pub fn rgb_to_hsv((r, g, b): (u8, u8, u8)) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta < f32::EPSILON {
        0.0
    } else if (max - r).abs() < f32::EPSILON {
        60.0 * (((g - b) / delta).rem_euclid(6.0))
    } else if (max - g).abs() < f32::EPSILON {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let s = if max < f32::EPSILON { 0.0 } else { delta / max };
    (h.rem_euclid(360.0), s, max)
}

pub fn color_list() {
    println!("Available colors:\n");
    println!("  Standard:  black  red  green  yellow  blue  magenta  cyan  white");
    println!("  Bright:    bright-black  bright-red  bright-green  bright-yellow");
    println!("             bright-blue  bright-magenta  bright-cyan  bright-white");
    println!("  Effects:   bold  dim  italic  underline  blink  inverse  strikethrough");
    println!("  Raw:       any ANSI code (e.g. 31, 91, 196)");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_code_standard() {
        assert_eq!(color_code("red"), Some("31"));
        assert_eq!(color_code("green"), Some("32"));
        assert_eq!(color_code("blue"), Some("34"));
        assert_eq!(color_code("white"), Some("37"));
        assert_eq!(color_code("black"), Some("30"));
    }

    #[test]
    fn test_color_code_bright() {
        assert_eq!(color_code("bright-red"), Some("91"));
        assert_eq!(color_code("bright-green"), Some("92"));
        assert_eq!(color_code("bright-cyan"), Some("96"));
    }

    #[test]
    fn test_color_code_effects() {
        assert_eq!(color_code("bold"), Some("1"));
        assert_eq!(color_code("dim"), Some("2"));
        assert_eq!(color_code("italic"), Some("3"));
        assert_eq!(color_code("underline"), Some("4"));
        assert_eq!(color_code("strikethrough"), Some("9"));
    }

    #[test]
    fn test_color_code_unknown() {
        assert_eq!(color_code("nope"), None);
        assert_eq!(color_code("rainbow"), None);
    }

    #[test]
    fn test_color_code_or_raw_named() {
        assert_eq!(color_code_or_raw("red"), Some("31".to_string()));
        assert_eq!(color_code_or_raw("bold"), Some("1".to_string()));
    }

    #[test]
    fn test_color_code_or_raw_numeric() {
        assert_eq!(color_code_or_raw("196"), Some("196".to_string()));
        assert_eq!(color_code_or_raw("31"), Some("31".to_string()));
    }

    #[test]
    fn test_color_code_or_raw_invalid() {
        assert_eq!(color_code_or_raw("nope"), None);
    }

    #[test]
    fn test_is_known_color() {
        assert!(is_known_color("red"));
        assert!(is_known_color("bright-blue"));
        assert!(is_known_color("bold"));
        assert!(!is_known_color("rainbow"));
        assert!(!is_known_color("196"));
    }

    #[test]
    fn test_urgency_ansi() {
        assert_eq!(urgency_ansi(0), "91");
        assert_eq!(urgency_ansi(1), "31");
        assert_eq!(urgency_ansi(2), "33");
        assert_eq!(urgency_ansi(5), "90");
        assert_eq!(urgency_ansi(9), "0");
    }

    #[test]
    fn test_urgency_dot() {
        assert!(!urgency_dot(0).is_empty());
        assert!(!urgency_dot(5).is_empty());
        assert!(urgency_dot(9).is_empty());
    }

    #[test]
    fn test_urgency_tab_color() {
        assert!(urgency_tab_color(0).is_some());
        assert!(urgency_tab_color(5).is_some());
        assert!(urgency_tab_color(6).is_none());
    }

    #[test]
    fn test_color_to_hex_named() {
        assert_eq!(color_to_hex("red"), Some("#CC3333"));
        assert_eq!(color_to_hex("dracula"), Some("#282A36"));
        assert_eq!(color_to_hex("nord"), Some("#2E3440"));
    }

    #[test]
    fn test_color_to_hex_unknown() {
        assert_eq!(color_to_hex("nonexistent"), None);
    }

    #[test]
    fn test_resolve_hex_passthrough() {
        assert_eq!(resolve_hex("#FF0000"), Some("#FF0000".to_string()));
        assert_eq!(resolve_hex("#123456"), Some("#123456".to_string()));
    }

    #[test]
    fn test_resolve_hex_named() {
        assert_eq!(resolve_hex("red"), Some("#CC3333".to_string()));
    }

    #[test]
    fn test_resolve_hex_invalid() {
        assert_eq!(resolve_hex("nope"), None);
    }

    #[test]
    fn test_hex_to_rgb_roundtrip() {
        assert_eq!(hex_to_rgb("#E8A23C"), Some((0xE8, 0xA2, 0x3C)));
        assert_eq!(rgb_to_hex((0xE8, 0xA2, 0x3C)), "#E8A23C");
        assert_eq!(hex_to_rgb("#zzzzzz"), None);
        assert_eq!(hex_to_rgb("#FFF"), None);
    }

    #[test]
    fn test_hsv_primaries() {
        assert_eq!(hsv_to_rgb(0.0, 1.0, 1.0), (255, 0, 0));
        assert_eq!(hsv_to_rgb(120.0, 1.0, 1.0), (0, 255, 0));
        assert_eq!(hsv_to_rgb(240.0, 1.0, 1.0), (0, 0, 255));
        assert_eq!(hsv_to_rgb(0.0, 0.0, 1.0), (255, 255, 255));
        assert_eq!(hsv_to_rgb(0.0, 0.0, 0.0), (0, 0, 0));
    }

    #[test]
    fn test_hsv_rgb_roundtrip() {
        for &hex in &["#E8A23C", "#1E1E2E", "#89B4FA", "#A6E3A1", "#F38BA8"] {
            let rgb = hex_to_rgb(hex).unwrap();
            let (h, s, v) = rgb_to_hsv(rgb);
            let back = hsv_to_rgb(h, s, v);
            // allow ±1 per channel for float rounding
            assert!((back.0 as i32 - rgb.0 as i32).abs() <= 1, "{hex} R");
            assert!((back.1 as i32 - rgb.1 as i32).abs() <= 1, "{hex} G");
            assert!((back.2 as i32 - rgb.2 as i32).abs() <= 1, "{hex} B");
        }
    }
}
