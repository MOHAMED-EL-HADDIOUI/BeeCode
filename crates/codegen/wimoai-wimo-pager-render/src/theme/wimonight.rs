//! BeeCode Hive theme: dark terminal surfaces with restrained honey accents.
//!
//! The canonical palette is defined in RGB (`Color::Rgb`).
//! At startup [`Theme::quantized`] downgrades every color to the terminal's detected capability level (256-color, 16-color, etc.).

use ratatui::style::{Color, Modifier};

use super::tokyonight::Theme;

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}

// Backgrounds and text use a custom grayscale ramp anchored at:
//   • bg  = #141414 (20)
//   • fg  = #f3f3f3 (243)
//
// Honey is reserved for focus, progress and intent; most of the interface stays dark.
#[allow(dead_code)]
mod palette {
    use super::*;

    // ── Backgrounds ─────────────────────────────────────────────────────
    pub const BG: Color = rgb(10, 10, 10); //  #0a0a0a, Night (terminal bg)
    pub const BG_DARK: Color = rgb(12, 12, 12); //  #0c0c0c, darkest
    pub const BG_STORM_DARK: Color = rgb(17, 17, 17); //  #111111, dark bg
    pub const BG_STORM: Color = rgb(20, 20, 20); //  #141414, main bg
    pub const BG_HIGHLIGHT: Color = rgb(36, 36, 36); //  #242424, highlight bg

    // ── Text / grays ────────────────────────────────────────────────────
    pub const FG: Color = rgb(225, 225, 225); // #e1e1e1, primary text
    pub const FG_DARK: Color = rgb(200, 200, 200); // #c8c8c8, secondary text
    pub const FG_GUTTER: Color = rgb(65, 65, 65); //  #414141, dim
    pub const COMMENT: Color = rgb(108, 108, 108); //  #6c6c6c, muted
    pub const DARK3: Color = rgb(90, 90, 90); //  #5a5a5a, medium gray
    pub const DARK5: Color = rgb(120, 120, 120); // #787878, bright gray

    // ── BeeCode palette ──────────────────────────────────────────────────
    pub const BLUE: Color = rgb(244, 185, 66); // #F4B942 honey gold
    pub const BLUE0: Color = rgb(146, 94, 18);
    pub const BLUE1: Color = rgb(255, 209, 102); // #FFD166 honey
    pub const CYAN: Color = rgb(250, 204, 21); // #FACC15 active yellow
    pub const GREEN: Color = rgb(74, 154, 104); // restrained success
    pub const GREEN1: Color = rgb(104, 176, 128);
    pub const MAGENTA: Color = rgb(245, 158, 11); // #F59E0B amber
    pub const ORANGE: Color = rgb(244, 185, 66);
    pub const PURPLE: Color = rgb(255, 209, 102);
    pub const RED: Color = rgb(220, 100, 92); // restrained error
    pub const RED1: Color = rgb(177, 66, 61);
    pub const TEAL: Color = rgb(244, 185, 66);
    pub const YELLOW: Color = rgb(245, 158, 11);

    pub const RED_DARK: Color = rgb(66, 14, 20); // #420e14, quantizes to 256-color red, not gray
    pub const GREEN_DARK: Color = rgb(6, 56, 6); // #063806, quantizes to 256-color green, not gray
}
use palette::*;

impl Theme {
    pub const fn wimonight() -> Self {
        Self {
            bg_base: rgb(24, 24, 27), // #18181B charcoal
            bg_light: BG_HIGHLIGHT,
            bg_dark: rgb(28, 28, 28), // lighter than bg_base for visible code blocks
            bg_highlight: BG_HIGHLIGHT,
            bg_hover: rgb(44, 44, 44),
            bg_terminal: rgb(9, 9, 11), // #09090B hive black

            accent_user: FG_DARK,
            accent_assistant: BLUE,
            accent_thinking: MAGENTA,
            accent_tool: DARK5,
            accent_system: BLUE,
            accent_error: RED,
            accent_success: GREEN,
            accent_running: MAGENTA,
            accent_skill: BLUE,

            text_primary: FG,
            text_secondary: FG_DARK,

            gray_dim: rgb(88, 88, 88), // #585858, slightly brighter than FG_GUTTER
            gray: COMMENT,
            gray_bright: DARK5,

            command: YELLOW,
            path: ORANGE,
            running: CYAN,
            warning: YELLOW,

            fuzzy_accent: BLUE,

            accent_plan: rgb(255, 219, 141), // #FFDB8D, golden

            accent_verify: rgb(187, 154, 247), // #bb9af7, violet

            accent_remember: Color::Rgb(139, 195, 74), // #8BC34A, Material Design light green

            selection_border: rgb(60, 60, 65),
            prompt_border: rgb(50, 50, 55), // #323237, dimmer prompt chrome
            prompt_border_active: rgb(80, 80, 88), // #505058, brighter when focused
            hover_border: rgb(30, 30, 34),

            accent_model: TEAL,

            scrollbar_bg: BG_STORM_DARK,
            scrollbar_fg: BG_HIGHLIGHT,

            diff_delete_bg: RED_DARK,
            diff_delete_fg: RED,
            diff_insert_bg: GREEN_DARK,
            diff_insert_fg: GREEN,
            diff_equal_fg: COMMENT,
            diff_gutter_fg: COMMENT,

            bg_visual: rgb(54, 54, 54),

            paste_bg: BG_STORM_DARK,
            paste_fg: FG_DARK,
            paste_dim: FG_GUTTER,

            md_heading_h1: TEAL,
            md_heading_h1_mod: Modifier::BOLD,
            md_heading_h2: BLUE,
            md_heading_h2_mod: Modifier::BOLD,
            md_heading_h3: PURPLE,
            md_heading_h3_mod: Modifier::BOLD,
            md_heading_h4: DARK5, // bright gray
            md_heading_h4_mod: Modifier::BOLD,
            md_heading_h5: COMMENT, // medium gray
            md_heading_h5_mod: Modifier::BOLD,
            md_heading_h6: DARK3, // medium gray, unbold
            md_heading_h6_mod: Modifier::empty(),
            md_code: BLUE1,
            md_task_checked: GREEN,
            md_task_unchecked: FG_DARK, // text_secondary
            md_muted: COMMENT,
            md_code_bg: rgb(28, 28, 28),
            md_text: FG_DARK,
            link_fg: rgb(122, 166, 218), // #7aa6da, soft blue for dark bg
        }
    }
}
