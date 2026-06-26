use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::io::{self, Write as IoWrite};
use std::time::{Duration, Instant};

use crate::theme::{self, BUILTIN_THEMES, THEME_META};

// ============================================================================
// Theme descriptions & tags
// ============================================================================

fn theme_meta(name: &str) -> (&'static str, &'static [&'static str]) {
    match name {
        // Editor Dark
        "catppuccin" => ("Warm pastel palette — Mocha variant", &["warm", "pastel"]),
        "catppuccin-frappe" => ("Catppuccin's medium-dark French roast", &["warm", "pastel"]),
        "catppuccin-macchiato" => ("Teal-accented twilight variant", &["warm", "pastel"]),
        "dracula" => (
            "Classic dark with vibrant neon accents",
            &["vibrant", "neon"],
        ),
        "nord" => ("Arctic north-bluish, low contrast", &["cool", "muted"]),
        "tokyo-night" => (
            "Dark Tokyo cityscape with neon highlights",
            &["cool", "vibrant"],
        ),
        "gruvbox" => ("Retro groove with warm earth tones", &["warm", "earth"]),
        "solarized-dark" => (
            "Ethan Schoonover's precision dark palette",
            &["warm", "muted"],
        ),
        "monokai" => (
            "Iconic Sublime Text palette, high contrast",
            &["vibrant", "high-contrast"],
        ),
        "one-dark" => ("Atom editor's signature dark palette", &["cool", "muted"]),
        "rose-pine" => ("Soho vibes with muted roses and golds", &["warm", "muted"]),
        "rose-pine-moon" => ("Rose Pine's deeper moonlit variant", &["cool", "muted"]),
        "kanagawa" => ("Inspired by Hokusai's Great Wave", &["warm", "earth"]),
        "everforest-dark" => ("Comfortable green forest palette", &["cool", "earth"]),
        "github-dark" => ("GitHub's official dark mode colors", &["cool", "muted"]),
        "ayu-dark" => ("Deep dark with warm golden accents", &["warm", "vibrant"]),
        "ayu-mirage" => ("Twilight variant, softer contrast", &["warm", "muted"]),
        "material-dark" => ("Google Material Design dark scheme", &["vibrant", "cool"]),
        // Editor Light
        "catppuccin-latte" => (
            "Catppuccin's lightest, airiest variant",
            &["warm", "pastel"],
        ),
        "nord-light" => ("Clean snowstorm light palette", &["cool", "muted"]),
        "tokyo-night-light" => ("Tokyo Night's bright daytime variant", &["cool", "muted"]),
        "gruvbox-light" => ("Gruvbox warmth on cream canvas", &["warm", "earth"]),
        "solarized-light" => ("Solarized on warm cream background", &["warm", "muted"]),
        "one-light" => ("Atom editor's clean light palette", &["cool", "muted"]),
        "rose-pine-dawn" => ("Warm morning light variant", &["warm", "muted"]),
        "kanagawa-lotus" => ("Light lotus pond variant", &["warm", "earth"]),
        "everforest-light" => ("Sun-dappled cream forest", &["cool", "earth"]),
        "github-light" => ("GitHub's classic light mode", &["cool", "muted"]),
        "ayu-light" => ("Bright, high-clarity variant", &["warm", "vibrant"]),
        "material-light" => ("Material Design light scheme", &["cool", "muted"]),
        "papercolor-light" => ("Crisp printer-paper look", &["cool", "muted"]),
        "pencil-light" => ("Muted sketch-pad palette", &["warm", "muted"]),
        "alabaster" => ("Minimal, almost-white backdrop", &["cool", "muted"]),
        // Chromatic Dark
        "amethyst" => ("Deep purple crystalline depths", &["cool", "vibrant"]),
        "cobalt" => ("Deep ocean blue midnight", &["cool", "vibrant"]),
        "merlot" => ("Rich burgundy wine tones", &["warm", "muted"]),
        "emerald" => ("Dense rainforest greens", &["cool", "earth"]),
        "copper" => ("Warm burnished metal hues", &["warm", "earth"]),
        "indigo-night" => ("Deep indigo twilight sky", &["cool", "vibrant"]),
        "plum" => ("Dark berry and violet tones", &["cool", "muted"]),
        "tundra" => ("Cold arctic blue-grey", &["cool", "muted"]),
        "mahogany" => ("Dark reddish-brown wood tones", &["warm", "earth"]),
        "olive-dusk" => ("Muted olive and sage at dusk", &["warm", "muted"]),
        "storm" => ("Steel grey thundercloud palette", &["cool", "muted"]),
        "volcanic" => ("Dark ember and ash tones", &["warm", "earth"]),
        // Chromatic Dark · by color
        "crimson" => (
            "Deep red background — bold and warm",
            &["red", "warm", "vibrant"],
        ),
        "tangerine" => (
            "Dark orange background — warm citrus",
            &["orange", "warm", "vibrant"],
        ),
        "amber" => (
            "Dark yellow-gold background — warm glow",
            &["yellow", "warm", "vibrant"],
        ),
        "lime" => (
            "Dark green background — fresh and vivid",
            &["green", "cool", "vibrant"],
        ),
        "teal" => (
            "Dark cyan background — deep aquamarine",
            &["cyan", "cool", "vibrant"],
        ),
        "azure" => (
            "Dark blue background — clear sky depths",
            &["blue", "cool", "vibrant"],
        ),
        "violet" => (
            "Dark purple background — rich and regal",
            &["purple", "cool", "vibrant"],
        ),
        "magenta" => (
            "Dark pink background — vivid berry",
            &["pink", "warm", "vibrant"],
        ),
        // Chromatic Light
        "terracotta" => ("Warm clay and earth on cream", &["warm", "earth"]),
        "sage-mist" => ("Soft sage green morning mist", &["cool", "muted"]),
        "lavender-mist" => ("Gentle purple-grey haze", &["cool", "pastel"]),
        "coral-sand" => ("Warm coral beach at sunset", &["warm", "vibrant"]),
        "arctic" => ("Ice blue on snow white", &["cool", "muted"]),
        "seafoam" => ("Cool ocean foam on mint", &["cool", "pastel"]),
        "champagne" => ("Warm golden bubbly tones", &["warm", "pastel"]),
        "blush" => ("Soft pink warmth on cream", &["warm", "pastel"]),
        // Chromatic Light · by color
        "crimson-light" => ("Soft red tint on pale rose", &["red", "warm", "pastel"]),
        "tangerine-light" => (
            "Soft orange tint on warm cream",
            &["orange", "warm", "pastel"],
        ),
        "amber-light" => (
            "Soft yellow tint on pale gold",
            &["yellow", "warm", "pastel"],
        ),
        "lime-light" => ("Soft green tint on pale leaf", &["green", "cool", "pastel"]),
        "teal-light" => ("Soft cyan tint on pale mint", &["cyan", "cool", "pastel"]),
        "azure-light" => ("Soft blue tint on pale sky", &["blue", "cool", "pastel"]),
        "violet-light" => (
            "Soft purple tint on pale lilac",
            &["purple", "cool", "pastel"],
        ),
        "magenta-light" => (
            "Soft pink tint on pale blossom",
            &["pink", "warm", "pastel"],
        ),
        // Semantic
        "danger" => (
            "Red alert — production / critical",
            &["warm", "high-contrast"],
        ),
        "safe" => ("Green all-clear — development", &["cool", "high-contrast"]),
        "ocean" => ("Deep ocean dark blues", &["cool", "muted"]),
        "ocean-light" => ("Coastal sky light blues", &["cool", "muted"]),
        "forest" => ("Dark woodland greens", &["cool", "earth"]),
        "forest-light" => ("Sunlit meadow greens", &["cool", "earth"]),
        "sunset" => ("Warm orange-gold dusk", &["warm", "vibrant"]),
        "dawn" => ("Soft peach morning light", &["warm", "pastel"]),
        "ivory" => ("Clean off-white elegance", &["warm", "muted"]),
        "slate" => ("Cool blue-grey professional", &["cool", "muted"]),
        _ => ("Custom user theme", &[]),
    }
}

const SLOT_NAMES: &[&str] = &[
    "bg",
    "fg",
    "cursor",
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
    "bright black",
    "bright red",
    "bright green",
    "bright yellow",
    "bright blue",
    "bright magenta",
    "bright cyan",
    "bright white",
];

const COLOR_SLOT_COUNT: usize = 19;
const SEMANTIC_SLOT_NAMES: &[&str] = &[
    "cursor_style",
    "cursor_blink",
    "title_style",
    "status_style",
    "critical_style",
    "urgent_style",
    "warning_style",
    "attention_style",
    "success_style",
    "muted_style",
];
const EDIT_SLOT_COUNT: usize = COLOR_SLOT_COUNT + SEMANTIC_SLOT_NAMES.len();

// ============================================================================
// Types
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
enum Region {
    EditorDark,
    EditorLight,
    ChromaticDark,
    ChromaticLight,
    Semantic,
    User,
}

impl Region {
    fn label(self) -> &'static str {
        match self {
            Region::EditorDark => "Editor Dark",
            Region::EditorLight => "Editor Light",
            Region::ChromaticDark => "Chromatic Dark",
            Region::ChromaticLight => "Chromatic Light",
            Region::Semantic => "Semantic",
            Region::User => "User",
        }
    }

    #[allow(dead_code)]
    fn style(self) -> Style {
        match self {
            Region::EditorDark | Region::EditorLight => Style::default().fg(Color::Yellow).bold(),
            Region::ChromaticDark | Region::ChromaticLight => {
                Style::default().fg(Color::Magenta).bold()
            }
            Region::Semantic => Style::default().fg(Color::Cyan).bold(),
            Region::User => Style::default().fg(Color::Green).bold(),
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Region::EditorDark => "■",
            Region::EditorLight => "☀",
            Region::ChromaticDark => "◆",
            Region::ChromaticLight => "◇",
            Region::Semantic => "●",
            Region::User => "★",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Browse,
    Search,
    Edit,
    Wheel,
    CreateName,
    Help,
}

struct ThemeEntry {
    name: String,
    region: Region,
    colors: Option<[String; 19]>,
}

/// One rendered row of the browse grid: a full-width section header, or a row
/// of up to `cols` theme cells. Theme values are indices into `order`.
enum VRow {
    Header(Region),
    Themes(Vec<usize>),
}

const REGIONS: [Region; 6] = [
    Region::EditorDark,
    Region::EditorLight,
    Region::ChromaticDark,
    Region::ChromaticLight,
    Region::Semantic,
    Region::User,
];

struct PickerState {
    entries: Vec<ThemeEntry>,
    /// entries indices, themes only, in region/display order. `cursor` indexes here.
    order: Vec<usize>,
    /// Browse grid rows for rendering + scrolling.
    vrows: Vec<VRow>,
    /// order-index -> (vrow index, column).
    pos: Vec<(usize, usize)>,
    /// Current column count for the browse grid.
    cols: usize,

    mode: ViewMode,
    cursor: usize,
    #[allow(dead_code)]
    scroll_offset: usize,

    search_text: String,
    search_results: Vec<usize>,
    search_cursor: usize,

    edit_entry_idx: usize,
    edit_colors: [String; 19],
    edit_semantics: Vec<String>,
    edit_slot: usize,
    edit_typing: bool,
    edit_input: String,
    edit_name: String,
    edit_modified: bool,

    create_name: String,

    // Color-wheel picker (HSV). Live-applies to edit_colors[edit_slot].
    wheel_h: f32,
    wheel_s: f32,
    wheel_v: f32,
    wheel_orig: String,

    #[allow(dead_code)]
    original_theme: Option<String>,
    selected: Option<String>,
    quit: bool,
    flash: Option<(String, Instant)>,
    ui: UiColors,
}

impl PickerState {
    fn current_theme_idx(&self) -> usize {
        match self.mode {
            ViewMode::Search => {
                if self.search_results.is_empty() {
                    0
                } else {
                    self.search_results[self.search_cursor.min(self.search_results.len() - 1)]
                }
            }
            _ => {
                if self.order.is_empty() {
                    0
                } else {
                    self.order[self.cursor.min(self.order.len() - 1)]
                }
            }
        }
    }

    fn current_name(&self) -> &str {
        &self.entries[self.current_theme_idx()].name
    }

    fn flash_msg(&mut self, msg: impl Into<String>) {
        self.flash = Some((msg.into(), Instant::now()));
    }

    fn active_flash(&self) -> Option<&str> {
        self.flash.as_ref().and_then(|(msg, t)| {
            if t.elapsed() < Duration::from_secs(4) {
                Some(msg.as_str())
            } else {
                None
            }
        })
    }

    fn update_search(&mut self) {
        let q = self.search_text.to_lowercase();
        self.search_results = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                let (desc, tags) = theme_meta(&e.name);
                e.name.to_lowercase().contains(&q)
                    || desc.to_lowercase().contains(&q)
                    || tags.iter().any(|t| t.contains(&q))
                    || e.region.label().to_lowercase().contains(&q)
            })
            .map(|(i, _)| i)
            .collect();
        self.search_cursor = 0;
    }

    fn browse_len(&self) -> usize {
        self.order.len()
    }

    fn search_len(&self) -> usize {
        self.search_results.len()
    }

    /// Recompute `order`, `vrows`, and `pos` from the current entries + cols.
    fn rebuild_grid(&mut self) {
        self.order = build_order(&self.entries);
        let (vrows, pos) = build_grid(&self.entries, &self.order, self.cols);
        self.vrows = vrows;
        self.pos = pos;
        if !self.order.is_empty() {
            self.cursor = self.cursor.min(self.order.len() - 1);
        } else {
            self.cursor = 0;
        }
    }
}

// ============================================================================
// Data construction
// ============================================================================

fn build_entries() -> Vec<ThemeEntry> {
    let mut entries = Vec::new();

    for (i, (name, colors)) in BUILTIN_THEMES.iter().enumerate() {
        let meta = &THEME_META[i];
        let region = match (meta.kind, meta.variant) {
            ("editor", "dark") => Region::EditorDark,
            ("editor", "light") => Region::EditorLight,
            ("chromatic", "dark") => Region::ChromaticDark,
            ("chromatic", "light") => Region::ChromaticLight,
            ("semantic", _) => Region::Semantic,
            _ => Region::EditorDark,
        };
        entries.push(ThemeEntry {
            name: name.to_string(),
            region,
            colors: Some(colors.map(|s| s.to_string())),
        });
    }

    let td = theme::theme_dir();
    if td.is_dir() {
        if let Ok(dir) = std::fs::read_dir(&td) {
            let mut user_names: Vec<String> = dir
                .flatten()
                .filter_map(|e| {
                    let p = e.path();
                    if p.extension().and_then(|x| x.to_str()) == Some("theme") {
                        p.file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            user_names.sort();
            for name in user_names {
                let colors = theme::load_user_theme(&name);
                entries.push(ThemeEntry {
                    name,
                    region: Region::User,
                    colors,
                });
            }
        }
    }

    entries
}

/// entries indices for every theme, grouped by region in display order.
fn build_order(entries: &[ThemeEntry]) -> Vec<usize> {
    let mut order = Vec::new();
    for region in REGIONS {
        for (i, e) in entries.iter().enumerate() {
            if e.region == region {
                order.push(i);
            }
        }
    }
    order
}

/// Lay `order` out into a column grid: a full-width header per region, then
/// rows of up to `cols` themes. Returns the visual rows and an order-index ->
/// (vrow, col) map for grid navigation.
fn build_grid(
    entries: &[ThemeEntry],
    order: &[usize],
    cols: usize,
) -> (Vec<VRow>, Vec<(usize, usize)>) {
    let cols = cols.max(1);
    let mut vrows: Vec<VRow> = Vec::new();
    let mut pos = vec![(0usize, 0usize); order.len()];

    let mut oi = 0;
    for region in REGIONS {
        let start = oi;
        while oi < order.len() && entries[order[oi]].region == region {
            oi += 1;
        }
        let end = oi;
        if start == end {
            continue;
        }
        vrows.push(VRow::Header(region));
        let mut k = start;
        while k < end {
            // The row we're about to fill will land at index `vrows.len()`.
            let vr = vrows.len();
            let mut row = Vec::new();
            for c in 0..cols {
                if k < end {
                    pos[k] = (vr, c);
                    row.push(k);
                    k += 1;
                }
            }
            vrows.push(VRow::Themes(row));
        }
    }

    (vrows, pos)
}

/// How many theme columns fit in the given total width. Capped for readability.
fn compute_cols(width: u16) -> usize {
    const CELL_MIN: u16 = 30;
    ((width / CELL_MIN) as usize).clamp(1, 2)
}

// ============================================================================
// Color helpers
// ============================================================================

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

fn hex_to_color(hex: &str) -> Color {
    hex_to_rgb(hex)
        .map(|(r, g, b)| Color::Rgb(r, g, b))
        .unwrap_or(Color::DarkGray)
}

fn swatch_spans<'a>(colors: &Option<[String; 19]>, width: usize) -> Vec<Span<'a>> {
    let Some(c) = colors else {
        return vec![Span::raw(" ".repeat(width))];
    };
    let indices = [0, 1, 4, 5, 7, 8, 9];
    let count = (width / 2).min(indices.len());
    indices[..count]
        .iter()
        .map(|&i| Span::styled("  ".to_string(), Style::default().bg(hex_to_color(&c[i]))))
        .collect()
}

fn is_valid_hex(s: &str) -> bool {
    s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}

// ============================================================================
// Theme-adaptive UI colors
// ============================================================================

struct UiColors {
    text: Color,
    text_dim: Color,
    text_muted: Color,
    accent: Color,
    accent_text: Color,
    border: Color,
    cat_a: Color,
    cat_b: Color,
    cat_c: Color,
    cat_d: Color,
}

fn blend_rgb(bg: (u8, u8, u8), fg: (u8, u8, u8), t: f32) -> Color {
    Color::Rgb(
        (bg.0 as f32 + (fg.0 as f32 - bg.0 as f32) * t) as u8,
        (bg.1 as f32 + (fg.1 as f32 - bg.1 as f32) * t) as u8,
        (bg.2 as f32 + (fg.2 as f32 - bg.2 as f32) * t) as u8,
    )
}

fn compute_ui_colors(colors: &Option<[String; 19]>) -> UiColors {
    let (bg, fg) = match colors {
        Some(c) => (
            hex_to_rgb(&c[0]).unwrap_or((30, 30, 46)),
            hex_to_rgb(&c[1]).unwrap_or((205, 214, 244)),
        ),
        None => ((30, 30, 46), (205, 214, 244)),
    };

    let bg_lum = 0.299 * bg.0 as f32 + 0.587 * bg.1 as f32 + 0.114 * bg.2 as f32;

    let ensure_contrast = |color: (u8, u8, u8)| -> Color {
        let delta = ((color.0 as i32 - bg.0 as i32).abs()
            + (color.1 as i32 - bg.1 as i32).abs()
            + (color.2 as i32 - bg.2 as i32).abs()) as f32
            / 3.0;
        if delta < 55.0 {
            blend_rgb(bg, fg, 0.75)
        } else {
            Color::Rgb(color.0, color.1, color.2)
        }
    };

    let palette_rgb = |idx: usize, fallback: (u8, u8, u8)| -> (u8, u8, u8) {
        match colors {
            Some(c) => hex_to_rgb(&c[idx]).unwrap_or(fallback),
            None => fallback,
        }
    };

    let accent_rgb = palette_rgb(
        6,
        if bg_lum > 128.0 {
            (180, 130, 10)
        } else {
            (249, 226, 175)
        },
    );
    let accent_lum =
        0.299 * accent_rgb.0 as f32 + 0.587 * accent_rgb.1 as f32 + 0.114 * accent_rgb.2 as f32;
    let accent_text = if accent_lum > 128.0 {
        Color::Rgb(20, 20, 30)
    } else {
        Color::Rgb(240, 240, 245)
    };

    UiColors {
        text: Color::Rgb(fg.0, fg.1, fg.2),
        text_dim: blend_rgb(bg, fg, 0.7),
        text_muted: blend_rgb(bg, fg, 0.4),
        accent: ensure_contrast(accent_rgb),
        accent_text,
        border: blend_rgb(bg, fg, 0.2),
        cat_a: ensure_contrast(palette_rgb(6, (249, 226, 175))),
        cat_b: ensure_contrast(palette_rgb(8, (245, 194, 231))),
        cat_c: ensure_contrast(palette_rgb(9, (148, 226, 213))),
        cat_d: ensure_contrast(palette_rgb(5, (166, 227, 161))),
    }
}

fn region_ui_color(region: Region, ui: &UiColors) -> Color {
    match region {
        Region::EditorDark | Region::EditorLight => ui.cat_a,
        Region::ChromaticDark | Region::ChromaticLight => ui.cat_b,
        Region::Semantic => ui.cat_c,
        Region::User => ui.cat_d,
    }
}

// ============================================================================
// Main loop
// ============================================================================

pub fn run_picker() {
    let entries = build_entries();
    if entries.is_empty() {
        eprintln!("No themes found.");
        return;
    }

    let order = build_order(&entries);
    let (vrows, pos) = build_grid(&entries, &order, 1);
    let original_theme = std::env::var("TAB_THEME").ok().filter(|s| !s.is_empty());

    let mut state = PickerState {
        entries,
        order,
        vrows,
        pos,
        cols: 1,
        mode: ViewMode::Browse,
        cursor: 0,
        scroll_offset: 0,
        search_text: String::new(),
        search_results: Vec::new(),
        search_cursor: 0,
        edit_entry_idx: 0,
        edit_colors: Default::default(),
        edit_semantics: vec![String::new(); SEMANTIC_SLOT_NAMES.len()],
        edit_slot: 0,
        edit_typing: false,
        edit_input: String::new(),
        edit_name: String::new(),
        edit_modified: false,
        create_name: String::new(),
        wheel_h: 0.0,
        wheel_s: 0.0,
        wheel_v: 0.0,
        wheel_orig: String::new(),
        original_theme: original_theme.clone(),
        selected: None,
        quit: false,
        flash: None,
        ui: compute_ui_colors(&None),
    };

    install_panic_hook();

    if enable_raw_mode().is_err() {
        eprintln!("Failed to enable raw mode");
        return;
    }
    let mut stdout = io::stdout();
    if execute!(stdout, EnterAlternateScreen).is_err() {
        let _ = disable_raw_mode();
        return;
    }

    let backend = CrosstermBackend::new(stdout);
    let Ok(mut terminal) = Terminal::new(backend) else {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
        return;
    };

    theme::apply_named_theme(state.current_name());
    {
        let idx = state.current_theme_idx();
        state.ui = compute_ui_colors(&state.entries[idx].colors);
    }

    loop {
        let cols = terminal.size().map(|s| compute_cols(s.width)).unwrap_or(1);
        if cols != state.cols {
            state.cols = cols;
            state.rebuild_grid();
        }

        let _ = terminal.draw(|f| ui(f, &state));

        if event::poll(Duration::from_millis(50)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == KeyEventKind::Press {
                    handle_key(&mut state, key);
                }
            }
        }

        if state.quit || state.selected.is_some() {
            break;
        }

        if state.mode != ViewMode::Edit && state.mode != ViewMode::Wheel {
            theme::apply_named_theme(state.current_name());
            let idx = state.current_theme_idx();
            state.ui = compute_ui_colors(&state.entries[idx].colors);
        }
    }

    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = disable_raw_mode();

    if state.quit {
        if let Some(ref orig) = original_theme {
            theme::apply_named_theme(orig);
        } else {
            theme::clear_theme();
        }
    } else if let Some(ref name) = state.selected {
        theme::apply_named_theme(name);
        std::env::set_var("TAB_THEME", name);
        prompt_save_envrc(name);
    }
}

fn install_panic_hook() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original(info);
    }));
}

// ============================================================================
// Input handling
// ============================================================================

fn handle_key(state: &mut PickerState, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        state.quit = true;
        return;
    }

    match state.mode {
        ViewMode::Browse => handle_browse(state, key),
        ViewMode::Search => handle_search(state, key),
        ViewMode::Edit => handle_edit(state, key),
        ViewMode::Wheel => handle_wheel(state, key),
        ViewMode::CreateName => handle_create(state, key),
        ViewMode::Help => handle_help(state, key),
    }
}

fn handle_browse(state: &mut PickerState, key: KeyEvent) {
    let len = state.browse_len();
    if len == 0 {
        return;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            state.quit = true;
        }
        KeyCode::Enter => {
            state.selected = Some(state.current_name().to_string());
        }
        KeyCode::Char('/') => {
            state.mode = ViewMode::Search;
            state.search_text.clear();
            state.update_search();
        }
        KeyCode::Left | KeyCode::Char('h') => {
            // Step back through themes in display order, wrapping to the last
            // entry from the first. Since `order` is contiguous per section,
            // this also steps back into the previous section at a boundary.
            state.cursor = if state.cursor == 0 {
                len - 1
            } else {
                state.cursor - 1
            };
        }
        KeyCode::Right | KeyCode::Char('l') => {
            // Step forward through themes in display order. At the end of a
            // section this lands on the first entry of the next section; at the
            // very end it wraps back to the first entry.
            state.cursor = if state.cursor + 1 >= len {
                0
            } else {
                state.cursor + 1
            };
        }
        KeyCode::Up | KeyCode::Char('k') => {
            move_grid_vertical(state, -1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            move_grid_vertical(state, 1);
        }
        KeyCode::PageUp => {
            for _ in 0..6 {
                move_grid_vertical(state, -1);
            }
        }
        KeyCode::PageDown => {
            for _ in 0..6 {
                move_grid_vertical(state, 1);
            }
        }
        KeyCode::Home | KeyCode::Char('g') => {
            state.cursor = 0;
        }
        KeyCode::End | KeyCode::Char('G') => {
            state.cursor = len - 1;
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            state.mode = ViewMode::CreateName;
            state.create_name.clear();
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            enter_edit_mode(state);
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            state.mode = ViewMode::CreateName;
            state.create_name.clear();
            state.flash_msg("Enter name for cloned theme");
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            try_delete_theme(state);
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            theme::clear_theme();
            state.flash_msg("Theme reset to terminal defaults");
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            state.selected = Some(state.current_name().to_string());
        }
        KeyCode::Char('?') => {
            state.mode = ViewMode::Help;
        }
        KeyCode::Tab => {
            jump_to_next_category(state, 1);
        }
        KeyCode::BackTab => {
            jump_to_next_category(state, -1);
        }
        _ => {}
    }
}

fn handle_search(state: &mut PickerState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.mode = ViewMode::Browse;
            state.search_text.clear();
        }
        KeyCode::Enter => {
            if !state.search_results.is_empty() {
                state.selected = Some(
                    state.entries[state.search_results[state.search_cursor]]
                        .name
                        .clone(),
                );
            }
        }
        KeyCode::Up => {
            state.search_cursor = state.search_cursor.saturating_sub(1);
        }
        KeyCode::Down => {
            if !state.search_results.is_empty() {
                state.search_cursor = (state.search_cursor + 1).min(state.search_results.len() - 1);
            }
        }
        KeyCode::Backspace => {
            if state.search_text.is_empty() {
                state.mode = ViewMode::Browse;
            } else {
                state.search_text.pop();
                state.update_search();
            }
        }
        KeyCode::Char(c) => {
            state.search_text.push(c);
            state.update_search();
        }
        _ => {}
    }
}

fn handle_edit(state: &mut PickerState, key: KeyEvent) {
    if state.edit_typing {
        match key.code {
            KeyCode::Esc => {
                state.edit_typing = false;
                state.edit_input.clear();
            }
            KeyCode::Enter => {
                if state.edit_slot >= COLOR_SLOT_COUNT {
                    let semantic_idx = state.edit_slot - COLOR_SLOT_COUNT;
                    if validate_semantic_input(semantic_idx, &state.edit_input) {
                        state.edit_semantics[semantic_idx] = state.edit_input.trim().to_string();
                        state.edit_modified = true;
                        state.edit_typing = false;
                        state.edit_input.clear();
                    } else {
                        state.flash_msg("Invalid style tokens");
                    }
                } else if is_valid_hex(&state.edit_input) {
                    state.edit_colors[state.edit_slot] = state.edit_input.clone();
                    state.edit_modified = true;
                    state.edit_typing = false;
                    apply_edit_colors(state);
                } else {
                    state.flash_msg("Invalid hex — use #RRGGBB format");
                }
            }
            KeyCode::Backspace => {
                state.edit_input.pop();
            }
            KeyCode::Char(c) => {
                if state.edit_slot >= COLOR_SLOT_COUNT {
                    if state.edit_input.len() < 64 {
                        state.edit_input.push(c);
                    }
                } else if state.edit_input.len() < 7 {
                    if state.edit_input.is_empty() && c != '#' {
                        state.edit_input.push('#');
                    }
                    state.edit_input.push(c.to_ascii_uppercase());
                    if is_valid_hex(&state.edit_input) {
                        state.edit_colors[state.edit_slot] = state.edit_input.clone();
                        state.edit_modified = true;
                        apply_edit_colors(state);
                    }
                }
            }
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            if state.edit_modified {
                state.flash_msg("Unsaved changes discarded");
            }
            state.mode = ViewMode::Browse;
        }
        KeyCode::Enter | KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char(' ') => {
            if state.edit_slot >= COLOR_SLOT_COUNT {
                let semantic_idx = state.edit_slot - COLOR_SLOT_COUNT;
                if is_select_semantic(semantic_idx) {
                    cycle_select_semantic(state, semantic_idx, 1);
                } else {
                    state.edit_typing = true;
                    state.edit_input = state.edit_semantics[semantic_idx].clone();
                }
            } else {
                open_wheel(state);
            }
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            if state.edit_slot >= COLOR_SLOT_COUNT {
                let semantic_idx = state.edit_slot - COLOR_SLOT_COUNT;
                if is_select_semantic(semantic_idx) {
                    cycle_select_semantic(state, semantic_idx, 1);
                } else {
                    state.edit_typing = true;
                    state.edit_input = state.edit_semantics[semantic_idx].clone();
                }
            } else {
                state.edit_typing = true;
                state.edit_input = state.edit_colors[state.edit_slot].clone();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.edit_slot = state.edit_slot.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.edit_slot = (state.edit_slot + 1).min(EDIT_SLOT_COUNT - 1);
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if state.edit_slot >= COLOR_SLOT_COUNT {
                let semantic_idx = state.edit_slot - COLOR_SLOT_COUNT;
                if is_select_semantic(semantic_idx) {
                    cycle_select_semantic(state, semantic_idx, -1);
                } else {
                    state.edit_slot = COLOR_SLOT_COUNT - 1;
                }
            } else if state.edit_slot >= 11 {
                state.edit_slot -= 8;
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if state.edit_slot >= COLOR_SLOT_COUNT {
                let semantic_idx = state.edit_slot - COLOR_SLOT_COUNT;
                if is_select_semantic(semantic_idx) {
                    cycle_select_semantic(state, semantic_idx, 1);
                }
            } else if state.edit_slot >= 3 && state.edit_slot <= 10 {
                state.edit_slot += 8;
            }
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            save_edit(state);
        }
        _ => {}
    }
}

fn validate_semantic_input(semantic_idx: usize, input: &str) -> bool {
    let input = input.trim();
    if input.is_empty() {
        return true;
    }
    match SEMANTIC_SLOT_NAMES.get(semantic_idx).copied().unwrap_or("") {
        "cursor_style" => matches!(
            input.replace('_', "-").to_ascii_lowercase().as_str(),
            "default"
                | "block"
                | "steady-block"
                | "blinking-block"
                | "bar"
                | "beam"
                | "steady-bar"
                | "steady-beam"
                | "blinking-bar"
                | "blinking-beam"
                | "underline"
                | "steady-underline"
                | "blinking-underline"
        ),
        "cursor_blink" => matches!(
            input.to_ascii_lowercase().as_str(),
            "true" | "false" | "1" | "0" | "yes" | "no" | "on" | "off"
        ),
        _ => crate::color::style_sequence(input).is_some(),
    }
}

fn is_select_semantic(semantic_idx: usize) -> bool {
    matches!(
        SEMANTIC_SLOT_NAMES.get(semantic_idx).copied().unwrap_or(""),
        "cursor_style" | "cursor_blink"
    )
}

fn semantic_options(semantic_idx: usize) -> &'static [&'static str] {
    match SEMANTIC_SLOT_NAMES.get(semantic_idx).copied().unwrap_or("") {
        "cursor_style" => &[
            "",
            "block",
            "bar",
            "underline",
            "blinking-block",
            "blinking-bar",
            "blinking-underline",
            "default",
        ],
        "cursor_blink" => &["", "true", "false"],
        _ => &[],
    }
}

fn cycle_select_semantic(state: &mut PickerState, semantic_idx: usize, dir: isize) {
    let options = semantic_options(semantic_idx);
    if options.is_empty() {
        return;
    }
    let current = state.edit_semantics[semantic_idx].trim();
    let cur_idx = options.iter().position(|v| *v == current).unwrap_or(0);
    let len = options.len() as isize;
    let next = (cur_idx as isize + dir).rem_euclid(len) as usize;
    state.edit_semantics[semantic_idx] = options[next].to_string();
    state.edit_modified = true;
}

fn open_wheel(state: &mut PickerState) {
    let slot = state.edit_slot;
    let hex = state.edit_colors[slot].clone();
    let rgb = crate::color::hex_to_rgb(&hex).unwrap_or((128, 128, 128));
    let (h, s, v) = crate::color::rgb_to_hsv(rgb);
    state.wheel_h = h;
    state.wheel_s = s;
    // A pure-black slot has value 0; nudge so the wheel is visible and editable.
    state.wheel_v = if v < 0.04 { 0.04 } else { v };
    state.wheel_orig = hex;
    state.edit_typing = false;
    state.edit_input.clear();
    state.mode = ViewMode::Wheel;
}

/// Current wheel HSV as a #RRGGBB string.
fn wheel_hex(state: &PickerState) -> String {
    crate::color::rgb_to_hex(crate::color::hsv_to_rgb(
        state.wheel_h,
        state.wheel_s,
        state.wheel_v,
    ))
}

/// Push the current wheel color into the edited slot and apply it live.
fn wheel_apply_live(state: &mut PickerState) {
    let slot = state.edit_slot;
    state.edit_colors[slot] = wheel_hex(state);
    state.edit_modified = true;
    apply_edit_colors(state);
}

/// Nudge the reticle in cartesian space (hue = angle, saturation = radius).
fn wheel_move(state: &mut PickerState, dx: f32, dy: f32) {
    let rad = state.wheel_h.to_radians();
    let mut x = state.wheel_s * rad.cos();
    let mut y = state.wheel_s * rad.sin();
    x = (x + dx).clamp(-1.0, 1.0);
    y = (y + dy).clamp(-1.0, 1.0);
    let mut s = (x * x + y * y).sqrt();
    if s > 1.0 {
        s = 1.0;
    }
    state.wheel_s = s;
    if s > 1e-4 {
        state.wheel_h = y.atan2(x).to_degrees().rem_euclid(360.0);
    }
}

fn handle_wheel(state: &mut PickerState, key: KeyEvent) {
    // Hex sub-entry: type an exact #RRGGBB without leaving the wheel.
    if state.edit_typing {
        match key.code {
            KeyCode::Esc => {
                state.edit_typing = false;
                state.edit_input.clear();
            }
            KeyCode::Enter => {
                if let Some(rgb) = crate::color::hex_to_rgb(&state.edit_input) {
                    let (h, s, v) = crate::color::rgb_to_hsv(rgb);
                    state.wheel_h = h;
                    state.wheel_s = s;
                    state.wheel_v = v;
                    state.edit_typing = false;
                    wheel_apply_live(state);
                } else {
                    state.flash_msg("Invalid hex — use #RRGGBB format");
                }
            }
            KeyCode::Backspace => {
                state.edit_input.pop();
            }
            KeyCode::Char(c) if state.edit_input.len() < 7 => {
                if state.edit_input.is_empty() && c != '#' {
                    state.edit_input.push('#');
                }
                state.edit_input.push(c.to_ascii_uppercase());
                if let Some(rgb) = crate::color::hex_to_rgb(&state.edit_input) {
                    let (h, s, v) = crate::color::rgb_to_hsv(rgb);
                    state.wheel_h = h;
                    state.wheel_s = s;
                    state.wheel_v = v;
                    wheel_apply_live(state);
                }
            }
            _ => {}
        }
        return;
    }

    const STEP: f32 = 0.07;
    const VSTEP: f32 = 0.04;
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            // Cancel: restore the slot to its pre-wheel value.
            let slot = state.edit_slot;
            state.edit_colors[slot] = state.wheel_orig.clone();
            apply_edit_colors(state);
            state.mode = ViewMode::Edit;
        }
        KeyCode::Enter => {
            wheel_apply_live(state);
            state.mode = ViewMode::Edit;
            state.flash_msg("Color set — press s to save theme");
        }
        KeyCode::Up | KeyCode::Char('k') => {
            wheel_move(state, 0.0, -STEP);
            wheel_apply_live(state);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            wheel_move(state, 0.0, STEP);
            wheel_apply_live(state);
        }
        KeyCode::Left | KeyCode::Char('h') => {
            wheel_move(state, -STEP, 0.0);
            wheel_apply_live(state);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            wheel_move(state, STEP, 0.0);
            wheel_apply_live(state);
        }
        KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::PageUp => {
            state.wheel_v = (state.wheel_v + VSTEP).min(1.0);
            wheel_apply_live(state);
        }
        KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::PageDown => {
            state.wheel_v = (state.wheel_v - VSTEP).max(0.0);
            wheel_apply_live(state);
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            state.edit_typing = true;
            state.edit_input = wheel_hex(state);
        }
        _ => {}
    }
}

fn handle_create(state: &mut PickerState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.mode = ViewMode::Browse;
        }
        KeyCode::Enter => {
            let name = state.create_name.trim().to_string();
            if name.is_empty() {
                state.flash_msg("Name cannot be empty");
                return;
            }
            if name.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_') {
                state.flash_msg("Name must be alphanumeric with - or _");
                return;
            }
            let td = theme::theme_dir();
            if td.join(format!("{}.theme", name)).exists() {
                state.flash_msg(format!("Theme '{}' already exists", name));
                return;
            }

            let base_idx = state.current_theme_idx();
            let base_colors = state.entries[base_idx]
                .colors
                .clone()
                .unwrap_or_else(default_colors);

            let _ = std::fs::create_dir_all(&td);
            let file = td.join(format!("{}.theme", name));
            let mut content = String::new();
            content.push_str(&format!("bg={}\n", base_colors[0]));
            content.push_str(&format!("fg={}\n", base_colors[1]));
            content.push_str(&format!("cursor={}\n", base_colors[2]));
            for i in 0..16 {
                content.push_str(&format!("color{}={}\n", i, base_colors[3 + i]));
            }
            let _ = std::fs::write(&file, &content);

            state.edit_entry_idx = state.entries.len();
            state.entries.push(ThemeEntry {
                name: name.clone(),
                region: Region::User,
                colors: Some(base_colors.clone()),
            });
            state.rebuild_grid();

            state.edit_colors = base_colors;
            state.edit_semantics = vec![String::new(); SEMANTIC_SLOT_NAMES.len()];
            state.edit_name = name;
            state.edit_slot = 0;
            state.edit_typing = false;
            state.edit_modified = false;
            state.mode = ViewMode::Edit;
            state.flash_msg("Theme created — edit colors below");
        }
        KeyCode::Backspace => {
            state.create_name.pop();
        }
        KeyCode::Char(c) if state.create_name.len() < 32 => {
            state.create_name.push(c);
        }
        _ => {}
    }
}

fn handle_help(state: &mut PickerState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Enter => {
            state.mode = ViewMode::Browse;
        }
        _ => {}
    }
}

// ============================================================================
// Actions
// ============================================================================

fn enter_edit_mode(state: &mut PickerState) {
    let idx = state.current_theme_idx();
    let entry = &state.entries[idx];

    if entry.region != Region::User {
        state.flash_msg("Clone first — press 'n' to create a variant");
        return;
    }

    let colors = entry.colors.clone().unwrap_or_else(default_colors);
    state.edit_entry_idx = idx;
    state.edit_colors = colors;
    state.edit_semantics = load_edit_semantics(&entry.name);
    state.edit_name = entry.name.clone();
    state.edit_slot = 0;
    state.edit_typing = false;
    state.edit_modified = false;
    state.mode = ViewMode::Edit;
}

fn apply_edit_colors(state: &PickerState) {
    let refs: [&str; 19] = std::array::from_fn(|i| state.edit_colors[i].as_str());
    theme::send_theme(&refs);
}

fn load_edit_semantics(name: &str) -> Vec<String> {
    let semantics = theme::load_user_theme_semantics(name);
    let mut values = vec![String::new(); SEMANTIC_SLOT_NAMES.len()];
    values[0] = semantics.cursor_style;
    values[1] = semantics
        .cursor_blink
        .map(|b| b.to_string())
        .unwrap_or_default();
    values[2] = semantics.title_style;
    values[3] = semantics.status_style;
    for i in 0..6 {
        values[4 + i] = semantics.urgency_styles[i].clone();
    }
    values
}

fn save_edit(state: &mut PickerState) {
    let td = theme::theme_dir();
    let file = td.join(format!("{}.theme", state.edit_name));
    let mut content = String::new();
    content.push_str(&format!("bg={}\n", state.edit_colors[0]));
    content.push_str(&format!("fg={}\n", state.edit_colors[1]));
    content.push_str(&format!("cursor={}\n", state.edit_colors[2]));
    for i in 0..16 {
        content.push_str(&format!("color{}={}\n", i, state.edit_colors[3 + i]));
    }
    for (idx, value) in state.edit_semantics.iter().enumerate() {
        if !value.trim().is_empty() {
            content.push_str(&format!("{}={}\n", SEMANTIC_SLOT_NAMES[idx], value.trim()));
        }
    }
    match std::fs::write(&file, &content) {
        Ok(_) => {
            state.entries[state.edit_entry_idx].colors = Some(state.edit_colors.clone());
            state.edit_modified = false;
            state.flash_msg(format!("Saved to {}", file.display()));
            state.mode = ViewMode::Browse;
        }
        Err(e) => {
            state.flash_msg(format!("Save failed: {}", e));
        }
    }
}

fn try_delete_theme(state: &mut PickerState) {
    let idx = state.current_theme_idx();
    if state.entries[idx].region != Region::User {
        state.flash_msg("Only user themes can be deleted");
        return;
    }

    let name = state.entries[idx].name.clone();
    let td = theme::theme_dir();
    let file = td.join(format!("{}.theme", name));
    if file.exists() {
        let _ = std::fs::remove_file(&file);
        state.entries.remove(idx);
        state.rebuild_grid();
        state.flash_msg(format!("Deleted '{}'", name));
    }
}

/// Move the cursor one grid row up (-1) or down (+1), keeping the column where
/// possible and skipping full-width section headers.
fn move_grid_vertical(state: &mut PickerState, dir: isize) {
    if state.order.is_empty() {
        return;
    }
    let cur = state.cursor.min(state.order.len() - 1);
    let (vr, col) = state.pos[cur];
    let mut tv = vr as isize;
    loop {
        tv += dir;
        if tv < 0 {
            // Past the top: loop to the very last theme.
            state.cursor = state.order.len() - 1;
            return;
        }
        if tv as usize >= state.vrows.len() {
            // Past the bottom: loop back to the first theme.
            state.cursor = 0;
            return;
        }
        if let VRow::Themes(row) = &state.vrows[tv as usize] {
            if row.is_empty() {
                continue;
            }
            let c = col.min(row.len() - 1);
            state.cursor = row[c];
            return;
        }
    }
}

fn jump_to_next_category(state: &mut PickerState, direction: i32) {
    if state.order.is_empty() {
        return;
    }
    let cur = state.cursor.min(state.order.len() - 1);
    let cur_region = state.entries[state.order[cur]].region;

    if direction > 0 {
        for j in (cur + 1)..state.order.len() {
            if state.entries[state.order[j]].region != cur_region {
                state.cursor = j;
                return;
            }
        }
    } else {
        for j in (0..cur).rev() {
            if state.entries[state.order[j]].region != cur_region {
                state.cursor = j;
                return;
            }
        }
    }
}

fn default_colors() -> [String; 19] {
    [
        "#1E1E2E", "#CDD6F4", "#F5E0DC", "#45475A", "#F38BA8", "#A6E3A1", "#F9E2AF", "#89B4FA",
        "#F5C2E7", "#94E2D5", "#BAC2DE", "#585B70", "#F38BA8", "#A6E3A1", "#F9E2AF", "#89B4FA",
        "#F5C2E7", "#94E2D5", "#A6ADC8",
    ]
    .map(|s| s.to_string())
}

// ============================================================================
// UI rendering
// ============================================================================

fn ui(f: &mut Frame, state: &PickerState) {
    let area = f.area();

    if state.mode == ViewMode::Help {
        render_help_overlay(f, area, state);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(5),
            Constraint::Length(4),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(f, chunks[0], state);

    match state.mode {
        ViewMode::Browse => render_browse(f, chunks[1], state),
        ViewMode::Search => render_search(f, chunks[1], state),
        ViewMode::Edit => render_edit(f, chunks[1], state),
        ViewMode::Wheel => {
            render_edit(f, chunks[1], state);
            render_wheel(f, area, state);
        }
        ViewMode::CreateName => {
            render_browse(f, chunks[1], state);
            render_create_modal(f, area, state);
        }
        ViewMode::Help => {}
    }

    render_preview(f, chunks[2], state);
    render_footer(f, chunks[3], state);
}

fn render_header(f: &mut Frame, area: Rect, state: &PickerState) {
    let mode_tabs: Vec<Span> = [
        ("Browse", ViewMode::Browse),
        ("Search", ViewMode::Search),
        ("Edit", ViewMode::Edit),
    ]
    .iter()
    .map(|(label, mode)| {
        if state.mode == *mode
            || (state.mode == ViewMode::CreateName && *mode == ViewMode::Browse)
            || (state.mode == ViewMode::Wheel && *mode == ViewMode::Edit)
        {
            Span::styled(
                format!(" {} ", label),
                Style::default()
                    .fg(state.ui.accent_text)
                    .bg(state.ui.accent)
                    .bold(),
            )
        } else {
            Span::styled(
                format!(" {} ", label),
                Style::default().fg(state.ui.text_muted),
            )
        }
    })
    .collect();

    let count = match state.mode {
        ViewMode::Search => format!("{} matches", state.search_len()),
        ViewMode::Edit | ViewMode::Wheel => format!("editing: {}", state.edit_name),
        _ => format!("{} themes", state.entries.len()),
    };

    let mut spans = mode_tabs;
    spans.push(Span::raw("  "));
    spans.push(Span::styled(
        count,
        Style::default().fg(state.ui.text_muted),
    ));

    if let Some(msg) = state.active_flash() {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            msg.to_string(),
            Style::default().fg(state.ui.accent).italic(),
        ));
    }

    let header = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .title(Span::styled(
                " tabbing-theme ",
                Style::default().fg(state.ui.text).bold(),
            ))
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(state.ui.border)),
    );

    f.render_widget(header, area);
}

fn render_browse(f: &mut Frame, area: Rect, state: &PickerState) {
    let visible = area.height as usize;
    if state.order.is_empty() || visible == 0 {
        return;
    }

    let cur = state.cursor.min(state.order.len() - 1);
    let cur_vrow = state.pos[cur].0;
    let scroll = compute_scroll(cur_vrow, visible, state.vrows.len());

    let cols = state.cols.max(1);
    let cell_w = (area.width as usize / cols).max(16);
    let swatch_w = if cell_w >= 34 { 12 } else { 8 };
    let name_w = cell_w.saturating_sub(3 + 1 + swatch_w + 1).max(6);

    let mut items: Vec<ListItem> = Vec::new();
    for vrow in state.vrows.iter().skip(scroll) {
        if items.len() >= visible {
            break;
        }
        match vrow {
            VRow::Header(region) => {
                let label = format!(" {} {} ", region.icon(), region.label());
                let dashes = area.width.saturating_sub(label.chars().count() as u16 + 1) as usize;
                let line = Line::from(vec![
                    Span::styled(
                        label,
                        Style::default()
                            .fg(region_ui_color(*region, &state.ui))
                            .bold(),
                    ),
                    Span::styled("─".repeat(dashes), Style::default().fg(state.ui.border)),
                ]);
                items.push(ListItem::new(line));
            }
            VRow::Themes(row) => {
                let mut spans: Vec<Span> = Vec::new();
                for &oi in row {
                    let entry = &state.entries[state.order[oi]];
                    let is_cursor = oi == cur;

                    let marker = if is_cursor { " ▸ " } else { "   " };
                    let name_style = if is_cursor {
                        Style::default()
                            .fg(state.ui.accent_text)
                            .bg(state.ui.accent)
                            .bold()
                            .add_modifier(Modifier::UNDERLINED | Modifier::SLOW_BLINK)
                    } else {
                        Style::default().fg(state.ui.text_dim)
                    };

                    let mut name = entry.name.clone();
                    if is_cursor {
                        name = format!("<<{}>>", name);
                    }
                    if name.chars().count() > name_w {
                        name.truncate(name_w.saturating_sub(1));
                        name.push('…');
                    }

                    spans.push(Span::styled(
                        marker.to_string(),
                        Style::default().fg(state.ui.accent),
                    ));
                    spans.push(Span::styled(
                        format!("{:<width$}", name, width = name_w),
                        name_style,
                    ));
                    spans.push(Span::raw(" "));
                    spans.extend(swatch_spans(&entry.colors, swatch_w));
                    spans.push(Span::raw(" "));
                }
                items.push(ListItem::new(Line::from(spans)));
            }
        }
    }

    f.render_widget(List::new(items), area);
}

fn render_search(f: &mut Frame, area: Rect, state: &PickerState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(3)])
        .split(area);

    let search_line = Line::from(vec![
        Span::styled(" / ", Style::default().fg(state.ui.accent).bold()),
        Span::styled(
            format!("{}▏", state.search_text),
            Style::default().fg(state.ui.text),
        ),
        Span::raw("  "),
        Span::styled(
            format!("{} results", state.search_results.len()),
            Style::default().fg(state.ui.text_muted),
        ),
    ]);
    let search_bar = Paragraph::new(search_line).block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(state.ui.border)),
    );
    f.render_widget(search_bar, chunks[0]);

    let visible = chunks[1].height as usize;
    let total = state.search_results.len();
    let cur = state.search_cursor.min(total.saturating_sub(1));
    let scroll = if cur >= visible { cur - visible + 1 } else { 0 };

    let wide = area.width > 80;
    let name_width = if wide { 22 } else { 18 };
    let swatch_width = if wide { 14 } else { 10 };

    let items: Vec<ListItem> = state
        .search_results
        .iter()
        .enumerate()
        .skip(scroll)
        .take(visible)
        .map(|(vi, &idx)| {
            let entry = &state.entries[idx];
            let (desc, _tags) = theme_meta(&entry.name);
            let is_cur = vi == cur;

            let marker = if is_cur { " ▸ " } else { "   " };
            let name_style = if is_cur {
                Style::default().fg(state.ui.text).bold()
            } else {
                Style::default().fg(state.ui.text_dim)
            };
            let tag = format!("[{}]", entry.region.label());

            let mut spans = vec![
                Span::styled(marker.to_string(), Style::default().fg(state.ui.accent)),
                Span::styled(
                    format!("{:<width$}", entry.name, width = name_width),
                    name_style,
                ),
                Span::raw(" "),
            ];
            spans.extend(swatch_spans(&entry.colors, swatch_width));
            spans.push(Span::styled(
                format!(" {:<18}", tag),
                Style::default().fg(state.ui.text_muted),
            ));
            if wide {
                spans.push(Span::styled(
                    desc.to_string(),
                    Style::default().fg(state.ui.text_muted),
                ));
            }
            ListItem::new(Line::from(spans))
        })
        .collect();

    if items.is_empty() {
        let empty = Paragraph::new(Span::styled(
            "   No themes match your search",
            Style::default().fg(state.ui.text_muted).italic(),
        ));
        f.render_widget(empty, chunks[1]);
    } else {
        f.render_widget(List::new(items), chunks[1]);
    }
}

fn render_edit(f: &mut Frame, area: Rect, state: &PickerState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Min(5),
        ])
        .split(chunks[0]);

    let meta_lines = vec![
        Line::from(vec![
            Span::styled("   bg       ", Style::default().fg(state.ui.text_muted)),
            render_edit_slot(state, 0),
        ]),
        Line::from(vec![
            Span::styled("   fg       ", Style::default().fg(state.ui.text_muted)),
            render_edit_slot(state, 1),
        ]),
        Line::from(vec![
            Span::styled("   cursor   ", Style::default().fg(state.ui.text_muted)),
            render_edit_slot(state, 2),
        ]),
    ];
    let meta = Paragraph::new(meta_lines).block(
        Block::default()
            .title(Span::styled(
                " Core ",
                Style::default().fg(state.ui.accent).bold(),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(state.ui.border)),
    );
    f.render_widget(meta, left_chunks[0]);

    let mut palette_lines: Vec<Line> = Vec::new();
    for i in 0..8 {
        let normal_slot = 3 + i;
        let bright_slot = 11 + i;
        palette_lines.push(Line::from(vec![
            Span::styled(
                format!("   {:<10}", SLOT_NAMES[normal_slot]),
                Style::default().fg(state.ui.text_muted),
            ),
            render_edit_slot(state, normal_slot),
            Span::raw("   "),
            Span::styled(
                format!("{:<14}", SLOT_NAMES[bright_slot]),
                Style::default().fg(state.ui.text_muted),
            ),
            render_edit_slot(state, bright_slot),
        ]));
    }
    let palette = Paragraph::new(palette_lines).block(
        Block::default()
            .title(Span::styled(
                " Palette ",
                Style::default().fg(state.ui.cat_b).bold(),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(state.ui.border)),
    );
    f.render_widget(palette, left_chunks[1]);

    let mut semantic_lines: Vec<Line> = Vec::new();
    for (i, name) in SEMANTIC_SLOT_NAMES.iter().enumerate() {
        semantic_lines.push(Line::from(vec![
            Span::styled(
                format!("   {:<15}", name),
                Style::default().fg(state.ui.text_muted),
            ),
            render_semantic_slot(state, i),
        ]));
    }
    let semantic = Paragraph::new(semantic_lines).block(
        Block::default()
            .title(Span::styled(
                " Semantic ",
                Style::default().fg(state.ui.cat_c).bold(),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(state.ui.border)),
    );
    f.render_widget(semantic, left_chunks[2]);

    render_code_preview(f, chunks[1], state);
}

fn render_semantic_slot<'a>(state: &PickerState, semantic_idx: usize) -> Span<'a> {
    let slot = COLOR_SLOT_COUNT + semantic_idx;
    let value = state
        .edit_semantics
        .get(semantic_idx)
        .map(|s| s.as_str())
        .unwrap_or("");
    let display = if value.is_empty() { "(default)" } else { value };
    let rendered = match SEMANTIC_SLOT_NAMES.get(semantic_idx).copied().unwrap_or("") {
        "cursor_style" => format!("[ {} ]", display),
        "cursor_blink" => match value {
            "true" => "[x] true".to_string(),
            "false" => "[ ] false".to_string(),
            _ => "[-] (default)".to_string(),
        },
        _ => display.to_string(),
    };
    let is_selected = state.edit_slot == slot;

    if is_selected && state.edit_typing {
        Span::styled(
            format!("{:<28}▏", state.edit_input),
            Style::default()
                .fg(state.ui.accent)
                .bg(state.ui.text_muted)
                .bold(),
        )
    } else if is_selected {
        Span::styled(
            format!("  {:<28} ◀", rendered),
            Style::default()
                .fg(state.ui.text)
                .bg(state.ui.accent)
                .bold()
                .add_modifier(Modifier::UNDERLINED),
        )
    } else {
        Span::styled(
            format!("  {:<28}  ", rendered),
            Style::default().fg(state.ui.text_dim),
        )
    }
}

fn render_edit_slot<'a>(state: &PickerState, slot: usize) -> Span<'a> {
    let hex = &state.edit_colors[slot];
    let is_selected = state.edit_slot == slot;

    if is_selected && state.edit_typing {
        Span::styled(
            format!("{:<7}▏ ", state.edit_input),
            Style::default()
                .fg(state.ui.accent)
                .bg(state.ui.text_muted)
                .bold(),
        )
    } else if is_selected {
        let swatch_bg = hex_to_color(hex);
        Span::styled(
            format!("  {} ◀", hex),
            Style::default().fg(state.ui.text).bg(swatch_bg).bold(),
        )
    } else {
        let swatch_bg = hex_to_color(hex);
        Span::styled(
            format!("  {}  ", hex),
            Style::default().fg(state.ui.text_dim).bg(swatch_bg),
        )
    }
}

fn render_code_preview(f: &mut Frame, area: Rect, state: &PickerState) {
    let c = &state.edit_colors;
    let bg = hex_to_color(&c[0]);
    let fg = hex_to_color(&c[1]);
    let red = hex_to_color(&c[4]);
    let green = hex_to_color(&c[5]);
    let yellow = hex_to_color(&c[6]);
    let blue = hex_to_color(&c[7]);
    let magenta = hex_to_color(&c[8]);
    let cyan = hex_to_color(&c[9]);
    let comment = hex_to_color(&c[11]);

    let base = Style::default().fg(fg).bg(bg);

    let code_lines = vec![
        Line::from(vec![
            Span::styled(" use ", Style::default().fg(magenta).bg(bg)),
            Span::styled("std::io", Style::default().fg(yellow).bg(bg)),
            Span::styled(";", base),
        ]),
        Line::from(Span::styled(" ", base)),
        Line::from(vec![
            Span::styled(" fn ", Style::default().fg(magenta).bg(bg)),
            Span::styled("main", Style::default().fg(blue).bg(bg)),
            Span::styled("() {", base),
        ]),
        Line::from(vec![
            Span::styled("     let ", Style::default().fg(magenta).bg(bg)),
            Span::styled("name", base),
            Span::styled(" = ", base),
            Span::styled("\"world\"", Style::default().fg(green).bg(bg)),
            Span::styled(";", base),
        ]),
        Line::from(vec![
            Span::styled("     let ", Style::default().fg(magenta).bg(bg)),
            Span::styled("count", base),
            Span::styled(" = ", base),
            Span::styled("42", Style::default().fg(yellow).bg(bg)),
            Span::styled(";", base),
        ]),
        Line::from(vec![
            Span::styled("     ", base),
            Span::styled("// greeting message", Style::default().fg(comment).bg(bg)),
        ]),
        Line::from(vec![
            Span::styled("     println!", Style::default().fg(cyan).bg(bg)),
            Span::styled("(", base),
            Span::styled("\"Hello {}!\"", Style::default().fg(green).bg(bg)),
            Span::styled(", name);", base),
        ]),
        Line::from(vec![
            Span::styled("     if ", Style::default().fg(magenta).bg(bg)),
            Span::styled("count ", base),
            Span::styled("> ", Style::default().fg(red).bg(bg)),
            Span::styled("0 ", Style::default().fg(yellow).bg(bg)),
            Span::styled("{", base),
        ]),
        Line::from(vec![
            Span::styled("         Ok", Style::default().fg(cyan).bg(bg)),
            Span::styled("(count)", base),
        ]),
        Line::from(vec![
            Span::styled("     } ", base),
            Span::styled("else ", Style::default().fg(magenta).bg(bg)),
            Span::styled("{", base),
        ]),
        Line::from(vec![
            Span::styled("         Err", Style::default().fg(red).bg(bg)),
            Span::styled("(", base),
            Span::styled("\"negative\"", Style::default().fg(green).bg(bg)),
            Span::styled(")", base),
        ]),
        Line::from(vec![Span::styled("     }", base)]),
        Line::from(vec![Span::styled(" }", base)]),
    ];

    let block = Block::default()
        .title(Span::styled(
            " Preview ",
            Style::default().fg(state.ui.cat_c).bold(),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(state.ui.border));

    let para = Paragraph::new(code_lines).block(block).style(base);
    f.render_widget(para, area);
}

fn render_preview(f: &mut Frame, area: Rect, state: &PickerState) {
    let idx = state.current_theme_idx();
    let entry = &state.entries[idx];
    let (desc, tags) = theme_meta(&entry.name);

    let tag_str: String = tags
        .iter()
        .map(|t| format!("#{}", t))
        .collect::<Vec<_>>()
        .join(" ");

    let colors = if matches!(state.mode, ViewMode::Edit | ViewMode::Wheel) {
        Some(state.edit_colors.clone())
    } else {
        entry.colors.clone()
    };

    let mut line1_spans = vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            entry.name.clone(),
            Style::default().fg(state.ui.text).bold(),
        ),
        Span::styled(
            format!("  [{}]  ", entry.region.label()),
            Style::default().fg(state.ui.text_muted),
        ),
    ];
    line1_spans.extend(swatch_spans(&colors, 14));

    let line2_spans = vec![
        Span::styled("  ", Style::default()),
        Span::styled(desc.to_string(), Style::default().fg(state.ui.text_dim)),
        Span::raw("  "),
        Span::styled(tag_str, Style::default().fg(state.ui.text_muted)),
    ];

    let code_preview = if let Some(ref c) = colors {
        let bg = hex_to_color(&c[0]);
        let fg = hex_to_color(&c[1]);
        let green = hex_to_color(&c[5]);
        let blue = hex_to_color(&c[7]);
        let magenta = hex_to_color(&c[8]);
        let base = Style::default().fg(fg).bg(bg);
        vec![
            Span::styled("  ", Style::default()),
            Span::styled(" fn ", Style::default().fg(magenta).bg(bg)),
            Span::styled("main", Style::default().fg(blue).bg(bg)),
            Span::styled("() { ", base),
            Span::styled("println!", Style::default().fg(blue).bg(bg)),
            Span::styled("(", base),
            Span::styled("\"hello\"", Style::default().fg(green).bg(bg)),
            Span::styled("); }", base),
            Span::styled(" ", Style::default()),
        ]
    } else {
        vec![Span::raw("")]
    };

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(state.ui.border));

    let para = Paragraph::new(vec![
        Line::from(line1_spans),
        Line::from(line2_spans),
        Line::from(code_preview),
    ])
    .block(block);

    f.render_widget(para, area);
}

fn render_footer(f: &mut Frame, area: Rect, state: &PickerState) {
    let keys: Vec<(&str, &str)> = match state.mode {
        ViewMode::Browse => vec![
            ("↑↓", "navigate"),
            ("Tab", "next group"),
            ("/", "search"),
            ("Enter", "apply"),
            ("n", "new"),
            ("e", "edit"),
            ("c", "clone"),
            ("d", "delete"),
            ("?", "help"),
        ],
        ViewMode::Search => vec![("↑↓", "navigate"), ("Enter", "apply"), ("Esc", "cancel")],
        ViewMode::Edit if state.edit_typing && state.edit_slot >= COLOR_SLOT_COUNT => vec![
            ("type", "style tokens"),
            ("Enter", "confirm"),
            ("Esc", "cancel"),
        ],
        ViewMode::Edit if state.edit_typing => {
            vec![("type", "#RRGGBB"), ("Enter", "confirm"), ("Esc", "cancel")]
        }
        ViewMode::Edit
            if state.edit_slot >= COLOR_SLOT_COUNT
                && is_select_semantic(state.edit_slot - COLOR_SLOT_COUNT) =>
        {
            vec![
                ("←→", "cycle"),
                ("Space", "toggle"),
                ("s", "save"),
                ("Esc", "cancel"),
            ]
        }
        ViewMode::Edit => vec![
            ("↑↓←→", "slots"),
            ("Enter", "edit"),
            ("x", "edit"),
            ("s", "save"),
            ("Esc", "cancel"),
        ],
        ViewMode::Wheel if state.edit_typing => {
            vec![("type", "#RRGGBB"), ("Enter", "confirm"), ("Esc", "cancel")]
        }
        ViewMode::Wheel => vec![
            ("↑↓←→", "move"),
            ("+/-", "value"),
            ("x", "hex"),
            ("Enter", "apply"),
            ("Esc", "cancel"),
        ],
        ViewMode::CreateName => vec![
            ("type", "theme name"),
            ("Enter", "create"),
            ("Esc", "cancel"),
        ],
        ViewMode::Help => vec![("?/Esc", "close help")],
    };

    let spans: Vec<Span> = keys
        .iter()
        .flat_map(|(k, desc)| {
            vec![
                Span::styled(
                    format!(" {} ", k),
                    Style::default()
                        .fg(state.ui.accent_text)
                        .bg(state.ui.text_muted)
                        .bold(),
                ),
                Span::styled(
                    format!(" {} ", desc),
                    Style::default().fg(state.ui.text_dim),
                ),
            ]
        })
        .collect();

    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_create_modal(f: &mut Frame, area: Rect, state: &PickerState) {
    let width = 44u16.min(area.width.saturating_sub(4));
    let height = 5u16;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let modal_area = Rect::new(x, y, width, height);

    let block = Block::default()
        .title(Span::styled(
            " New Theme ",
            Style::default().fg(state.ui.accent).bold(),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(state.ui.accent));

    f.render_widget(Clear, modal_area);

    let inner = block.inner(modal_area);
    f.render_widget(block, modal_area);

    let base_name = state.entries[state.current_theme_idx()].name.clone();
    let lines = vec![
        Line::from(vec![
            Span::styled(" Name: ", Style::default().fg(state.ui.text_muted)),
            Span::styled(
                format!("{}▏", state.create_name),
                Style::default().fg(state.ui.text).bold(),
            ),
        ]),
        Line::from(vec![Span::styled(
            format!(" Based on: {}", base_name),
            Style::default().fg(state.ui.text_muted),
        )]),
    ];

    f.render_widget(Paragraph::new(lines), inner);
}

fn render_wheel(f: &mut Frame, area: Rect, state: &PickerState) {
    let ui = &state.ui;
    let slot_name = SLOT_NAMES.get(state.edit_slot).copied().unwrap_or("color");

    let width = 60u16.min(area.width.saturating_sub(2));
    let height = 22u16.min(area.height.saturating_sub(1));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let modal = Rect::new(x, y, width, height);

    f.render_widget(Clear, modal);

    let block = Block::default()
        .title(Span::styled(
            format!(" Color Wheel · {} ", slot_name),
            Style::default().fg(ui.accent).bold(),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ui.accent));
    let inner = block.inner(modal);
    f.render_widget(block, modal);

    // Disc geometry: C = 2R so a unit circle renders round (cells are ~2:1).
    let slider_w: i32 = 11;
    let avail_w = inner.width as i32;
    let avail_h = inner.height as i32;
    let mut r = (avail_h - 6).clamp(5, 11);
    let mut c = r * 2;
    if c + slider_w > avail_w {
        c = (avail_w - slider_w).max(6);
        r = (c / 2).clamp(4, 11);
        c = r * 2;
    }
    if r < 2 || c < 2 {
        return;
    }

    let value = state.wheel_v;
    let rad = state.wheel_h.to_radians();
    let rx = state.wheel_s * rad.cos();
    let ry = state.wheel_s * rad.sin();
    let ret_col = ((rx * (c as f32 / 2.0) + c as f32 / 2.0 - 0.5).round() as i32).clamp(0, c - 1);
    let ret_row = ((ry * (r as f32 / 2.0) + r as f32 / 2.0 - 0.5).round() as i32).clamp(0, r - 1);
    let slider_marker = ((1.0 - value) * (r - 1) as f32).round() as i32;

    let mut lines: Vec<Line> = Vec::new();
    for row in 0..r {
        let mut spans: Vec<Span> = Vec::new();
        for col in 0..c {
            let nx = (col as f32 + 0.5 - c as f32 / 2.0) / (c as f32 / 2.0);
            let ny = (row as f32 + 0.5 - r as f32 / 2.0) / (r as f32 / 2.0);
            let dist = (nx * nx + ny * ny).sqrt();
            if dist <= 1.0 {
                let hue = ny.atan2(nx).to_degrees().rem_euclid(360.0);
                let (cr, cg, cb) = crate::color::hsv_to_rgb(hue, dist, value);
                if col == ret_col && row == ret_row {
                    let lum = 0.299 * cr as f32 + 0.587 * cg as f32 + 0.114 * cb as f32;
                    let fg = if lum > 140.0 {
                        Color::Black
                    } else {
                        Color::White
                    };
                    spans.push(Span::styled(
                        "◉",
                        Style::default().fg(fg).bg(Color::Rgb(cr, cg, cb)),
                    ));
                } else {
                    spans.push(Span::styled(
                        " ",
                        Style::default().bg(Color::Rgb(cr, cg, cb)),
                    ));
                }
            } else {
                spans.push(Span::raw(" "));
            }
        }

        // Value slider (top = brightest).
        spans.push(Span::raw(" "));
        let v_level = 1.0 - row as f32 / (r - 1) as f32;
        let (sr, sg, sb) =
            crate::color::hsv_to_rgb(state.wheel_h, state.wheel_s.max(0.0001), v_level);
        if row == slider_marker {
            spans.push(Span::styled("◀", Style::default().fg(ui.accent).bold()));
        } else {
            spans.push(Span::raw(" "));
        }
        spans.push(Span::styled(
            "    ",
            Style::default().bg(Color::Rgb(sr, sg, sb)),
        ));
        if row == 0 {
            spans.push(Span::styled(" Val", Style::default().fg(ui.text_muted)));
        } else if row == 1 {
            spans.push(Span::styled(
                format!(" {:>3}%", (value * 100.0).round() as i32),
                Style::default().fg(ui.text_dim),
            ));
        }

        lines.push(Line::from(spans));
    }

    // Readout: live swatch + hex + H/S/V.
    let hex = wheel_hex(state);
    let (rr, rg, rb) = crate::color::hsv_to_rgb(state.wheel_h, state.wheel_s, state.wheel_v);
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("    ", Style::default().bg(Color::Rgb(rr, rg, rb))),
        Span::styled(format!("  {}   ", hex), Style::default().fg(ui.text).bold()),
        Span::styled(
            format!(
                "H {:>3}°  S {:>3}%  V {:>3}%",
                state.wheel_h.round() as i32,
                (state.wheel_s * 100.0).round() as i32,
                (state.wheel_v * 100.0).round() as i32
            ),
            Style::default().fg(ui.text_muted),
        ),
    ]));

    if state.edit_typing {
        lines.push(Line::from(vec![
            Span::styled("  hex ", Style::default().fg(ui.text_muted)),
            Span::styled(
                format!("{}▏", state.edit_input),
                Style::default().fg(ui.accent).bold(),
            ),
        ]));
    } else {
        // Palette context: 16 ANSI swatches, current slot marked.
        let mut pspans = vec![Span::raw("  ")];
        for i in 0..16usize {
            let slot = 3 + i;
            let col = crate::color::hex_to_rgb(&state.edit_colors[slot]).unwrap_or((0, 0, 0));
            pspans.push(Span::styled(
                if slot == state.edit_slot { "▼" } else { " " },
                Style::default().fg(ui.accent),
            ));
            pspans.push(Span::styled(
                " ",
                Style::default().bg(Color::Rgb(col.0, col.1, col.2)),
            ));
        }
        lines.push(Line::from(pspans));
    }

    f.render_widget(Paragraph::new(lines), inner);
}

fn render_help_overlay(f: &mut Frame, area: Rect, state: &PickerState) {
    let width = 56u16.min(area.width.saturating_sub(4));
    let height = 22u16.min(area.height.saturating_sub(2));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let modal_area = Rect::new(x, y, width, height);

    f.render_widget(Clear, modal_area);

    let block = Block::default()
        .title(Span::styled(
            " Keyboard Shortcuts ",
            Style::default().fg(state.ui.accent).bold(),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(state.ui.accent));

    let inner = block.inner(modal_area);
    f.render_widget(block, modal_area);

    let accent = state.ui.accent;
    let key_color = state.ui.cat_c;
    let desc_color = state.ui.text_dim;
    let h = move |k: &str, d: &str| -> Line {
        Line::from(vec![
            Span::styled(format!("   {:<14}", k), Style::default().fg(key_color)),
            Span::styled(d.to_string(), Style::default().fg(desc_color)),
        ])
    };

    let lines = vec![
        Line::from(Span::styled(
            " Navigation",
            Style::default().fg(accent).bold(),
        )),
        h("↑/k  ↓/j", "Move up / down a row"),
        h("←/h  →/l", "Move left / right a column"),
        h("Tab/S-Tab", "Next / previous category"),
        h("PgUp/PgDn", "Page up / down"),
        h("Home/End", "First / last theme"),
        Line::from(""),
        Line::from(Span::styled(" Actions", Style::default().fg(accent).bold())),
        h("Enter", "Apply selected theme"),
        h("/", "Search (name, desc, tags)"),
        h("n", "New theme (clone + edit)"),
        h("e", "Edit colors (⏎ opens color wheel)"),
        h("c", "Clone to user themes"),
        h("d", "Delete user theme"),
        h("s", "Apply & save to .envrc"),
        h("r", "Reset to terminal defaults"),
        h("?", "Toggle this help"),
        h("q / Esc", "Quit (restore original)"),
        Line::from(""),
        Line::from(Span::styled(
            "       Press ? or Esc to close",
            Style::default().fg(state.ui.text_muted),
        )),
    ];

    f.render_widget(Paragraph::new(lines), inner);
}

/// Top vrow index to render so that `cursor_vrow` stays visible with a little
/// context above it.
fn compute_scroll(cursor_vrow: usize, visible: usize, total_vrows: usize) -> usize {
    if visible == 0 || total_vrows <= visible {
        return 0;
    }
    let context = 2usize;
    if cursor_vrow < visible {
        0
    } else {
        cursor_vrow
            .saturating_sub(context)
            .min(total_vrows - visible)
    }
}

// ============================================================================
// .envrc integration
// ============================================================================

fn prompt_save_envrc(theme_name: &str) {
    println!("Theme \"{}\" applied.", theme_name);
    print!("Save to .envrc? [y/N] ");
    let _ = io::stdout().flush();

    let mut answer = String::new();
    if io::stdin().read_line(&mut answer).is_ok() {
        if answer.trim().eq_ignore_ascii_case("y") {
            save_theme_to_envrc(theme_name, ".");
        } else {
            println!(
                "Theme active for this session only (export TAB_THEME={}).",
                theme_name
            );
        }
    }
}

fn save_theme_to_envrc(theme_name: &str, dir: &str) {
    let envrc = format!("{}/.envrc", dir);
    let line = format!("export TAB_THEME={}", theme_name);

    if let Ok(contents) = std::fs::read_to_string(&envrc) {
        if contents.contains("export TAB_THEME=") {
            let updated: String = contents
                .lines()
                .map(|l| {
                    if l.starts_with("export TAB_THEME=") {
                        line.as_str()
                    } else {
                        l
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            let _ = std::fs::write(&envrc, format!("{}\n", updated));
            println!("Updated TAB_THEME={} in {}", theme_name, envrc);
        } else {
            let _ = std::fs::OpenOptions::new()
                .append(true)
                .open(&envrc)
                .and_then(|mut f| writeln!(f, "\n{}", line));
            println!("Added TAB_THEME={} to {}", theme_name, envrc);
        }
    } else {
        let _ = std::fs::write(&envrc, format!("{}\n", line));
        println!("Created {} with TAB_THEME={}", envrc, theme_name);
    }

    if which_exists("direnv") {
        println!("Run \"direnv allow\" to activate.");
    }
}

fn which_exists(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// ============================================================================
// Public CLI helpers
// ============================================================================

pub fn clone_theme(src: &str, dest: &str) {
    let td = theme::theme_dir();
    let _ = std::fs::create_dir_all(&td);
    let dest_path = td.join(format!("{}.theme", dest));

    let src_path = td.join(format!("{}.theme", src));
    if src_path.exists() {
        match std::fs::copy(&src_path, &dest_path) {
            Ok(_) => {
                println!("Cloned user theme \"{}\" → \"{}\"", src, dest);
                println!("  {}", dest_path.display());
            }
            Err(e) => eprintln!("Failed to clone: {}", e),
        }
        return;
    }

    if let Some(colors) = find_builtin_colors(src) {
        let mut content = String::new();
        content.push_str(&format!("bg={}\n", colors[0]));
        content.push_str(&format!("fg={}\n", colors[1]));
        content.push_str(&format!("cursor={}\n", colors[2]));
        for i in 0..16 {
            content.push_str(&format!("color{}={}\n", i, colors[3 + i]));
        }
        match std::fs::write(&dest_path, &content) {
            Ok(_) => {
                println!("Cloned built-in theme \"{}\" → \"{}\"", src, dest);
                println!("  {}", dest_path.display());
            }
            Err(e) => eprintln!("Failed to write: {}", e),
        }
    } else {
        eprintln!("Unknown theme: {}", src);
    }
}

pub fn edit_theme(name: &str) {
    let td = theme::theme_dir();
    let file = td.join(format!("{}.theme", name));

    if !file.exists() {
        eprintln!("Theme \"{}\" is not a user theme.", name);
        eprintln!("Clone it first: tabbing-theme clone {} my-{}", name, name);
        return;
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());
    let status = std::process::Command::new(&editor).arg(&file).status();

    match status {
        Ok(s) if s.success() => {
            print!("Preview updated theme? [Y/n] ");
            let _ = io::stdout().flush();
            let mut ans = String::new();
            if io::stdin().read_line(&mut ans).is_ok() {
                if !ans.trim().eq_ignore_ascii_case("n") {
                    if theme::apply_named_theme(name) {
                        println!("Theme \"{}\" applied.", name);
                    }
                }
            }
        }
        _ => eprintln!("Failed to open editor: {}", editor),
    }
}

pub fn delete_theme(name: &str) {
    let td = theme::theme_dir();
    let file = td.join(format!("{}.theme", name));

    if !file.exists() {
        eprintln!("No user theme \"{}\" found.", name);
        return;
    }

    print!("Delete theme \"{}\"? [y/N] ", name);
    let _ = io::stdout().flush();
    let mut ans = String::new();
    if io::stdin().read_line(&mut ans).is_ok() {
        if ans.trim().eq_ignore_ascii_case("y") {
            let _ = std::fs::remove_file(&file);
            println!("Deleted.");
        } else {
            println!("Cancelled.");
        }
    }
}

pub fn export_theme(name: &str) {
    if let Some(colors) = find_builtin_colors(name) {
        println!("bg={}", colors[0]);
        println!("fg={}", colors[1]);
        println!("cursor={}", colors[2]);
        for i in 0..16 {
            println!("color{}={}", i, colors[3 + i]);
        }
        return;
    }

    if let Some(colors) = theme::load_user_theme(name) {
        println!("bg={}", colors[0]);
        println!("fg={}", colors[1]);
        println!("cursor={}", colors[2]);
        for i in 0..16 {
            println!("color{}={}", i, colors[3 + i]);
        }
        return;
    }

    eprintln!("Unknown theme: {}", name);
    std::process::exit(1);
}

pub fn preview_theme(name: &str) {
    if theme::apply_named_theme(name) {
        std::env::set_var("TAB_THEME", name);
        println!(
            "Previewing theme \"{}\" — use \"tabbing-theme reset\" to undo.",
            name
        );
    } else {
        eprintln!("Unknown theme: {}", name);
        std::process::exit(1);
    }
}

fn find_builtin_colors(name: &str) -> Option<Vec<String>> {
    let canonical = crate::theme::resolve_alias_pub(name);
    BUILTIN_THEMES
        .iter()
        .find(|(n, _)| *n == canonical)
        .map(|(_, c)| c.iter().map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_places_every_theme_exactly_once() {
        let entries = build_entries();
        let order = build_order(&entries);
        assert!(!order.is_empty());

        for cols in 1..=4usize {
            let (vrows, pos) = build_grid(&entries, &order, cols);
            assert_eq!(pos.len(), order.len());

            // Collect every order-index that appears in a Themes row.
            let mut seen = vec![false; order.len()];
            let mut header_count = 0usize;
            for (vi, vrow) in vrows.iter().enumerate() {
                match vrow {
                    VRow::Header(_) => header_count += 1,
                    VRow::Themes(row) => {
                        assert!(row.len() <= cols, "row wider than {cols} cols");
                        for (c, &oi) in row.iter().enumerate() {
                            assert!(!seen[oi], "order idx {oi} placed twice");
                            seen[oi] = true;
                            // pos must point back to this exact cell.
                            assert_eq!(pos[oi], (vi, c));
                        }
                    }
                }
            }
            assert!(
                seen.iter().all(|&b| b),
                "some theme not placed at cols={cols}"
            );
            assert!(header_count >= 5, "expected a header per populated region");
        }
    }

    #[test]
    fn compute_cols_scales_and_clamps() {
        assert_eq!(compute_cols(20), 1);
        assert_eq!(compute_cols(60), 2);
        assert_eq!(compute_cols(95), 3);
        assert_eq!(compute_cols(400), 4); // capped
    }

    fn test_state(mode: ViewMode, cols: usize) -> PickerState {
        let entries = build_entries();
        let order = build_order(&entries);
        let (vrows, pos) = build_grid(&entries, &order, cols);
        PickerState {
            entries,
            order,
            vrows,
            pos,
            cols,
            mode,
            cursor: 5,
            scroll_offset: 0,
            search_text: "blue".into(),
            search_results: vec![0, 1, 2],
            search_cursor: 1,
            edit_entry_idx: 0,
            edit_colors: default_colors(),
            edit_semantics: vec![String::new(); SEMANTIC_SLOT_NAMES.len()],
            edit_slot: 6,
            edit_typing: false,
            edit_input: String::new(),
            edit_name: "test".into(),
            edit_modified: false,
            create_name: String::new(),
            wheel_h: 36.0,
            wheel_s: 0.74,
            wheel_v: 0.91,
            wheel_orig: "#1E1E2E".into(),
            original_theme: None,
            selected: None,
            quit: false,
            flash: None,
            ui: compute_ui_colors(&None),
        }
    }

    #[test]
    fn renders_every_mode_without_panic() {
        use ratatui::backend::TestBackend;
        use ratatui::Terminal;

        for &(mode, cols) in &[
            (ViewMode::Browse, 3),
            (ViewMode::Browse, 1),
            (ViewMode::Search, 3),
            (ViewMode::Edit, 3),
            (ViewMode::Wheel, 3),
            (ViewMode::Help, 3),
        ] {
            let state = test_state(mode, cols);
            // A small terminal also exercises the saturating/clamp paths.
            for (w, h) in [(100u16, 30u16), (40, 14)] {
                let backend = TestBackend::new(w, h);
                let mut term = Terminal::new(backend).unwrap();
                term.draw(|f| ui(f, &state)).unwrap();
            }
        }
    }
}
