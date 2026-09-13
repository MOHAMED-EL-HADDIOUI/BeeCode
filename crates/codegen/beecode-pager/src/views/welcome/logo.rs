//! Terminal-native BeeCode hive animation.
//!
//! The welcome view keeps its existing layout; this module only owns the visual
//! mark. Modern terminals receive a six-frame honeycomb with a worker in
//! flight and a low-cost color pulse. Legacy Windows consoles use ASCII frames.

use std::sync::OnceLock;
use std::time::Instant;

use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::render::color::blend_color;
use crate::theme::Theme;

const SMALL_LOGO_MIN_HEIGHT: u16 = 22;
const FULL_LOGO_MIN_HEIGHT: u16 = 26;
const ANIMATION_FPS: f32 = 8.0;
const FRAME_COUNT: usize = 6;

const CORE_FRAMES: [[&str; 7]; FRAME_COUNT] = [
    [
        "            ·   ✦   ·            ",
        "       ╭────◌───────◌────╮       ",
        "    ·  │   ╲    ·    ╱   │  ·    ",
        "       ◌─────╲  ◉  ╱─────◌       ",
        "    ·  │   ╱    ·    ╲   │  ·    ",
        "       ╰────◌───────◌────╯       ",
        "            ·   ·   ·            ",
    ],
    [
        "            ·   ·   ✦            ",
        "       ╭────◌───────◌────╮       ",
        "       │   ╱    ·    ╲   │  ·    ",
        "    ·  ◌─────╱  ◉  ╲─────◌       ",
        "       │   ╲    ·    ╱   │  ·    ",
        "       ╰────◌───────◌────╯       ",
        "            ·   ·   ·            ",
    ],
    [
        "            ✦   ·   ·            ",
        "       ╭────◌───────◌────╮       ",
        "       │   ╲    ·    ╱   │       ",
        "    ·  ◌─────╲  ●  ╱─────◌  ·    ",
        "       │   ╱    ·    ╲   │       ",
        "       ╰────◌───────◌────╯       ",
        "            ·   ·   ✦            ",
    ],
    [
        "            ·   ·   ·            ",
        "       ╭────◌─────✦─◌────╮       ",
        "       │   ╱    ·    ╲   │       ",
        "    ·  ◌─────╱  ◉  ╲─────◌  ·    ",
        "       │   ╲    ·    ╱   │       ",
        "       ╰────◌───────◌────╯       ",
        "            ✦   ·   ·            ",
    ],
    [
        "            ·   ·   ·            ",
        "       ╭────◌───────◌────╮       ",
        "       │   ╲    ✦    ╱   │       ",
        "    ·  ◌─────╲  ◉  ╱─────◌  ·    ",
        "       │   ╱    ·    ╲   │       ",
        "       ╰────◌───────◌────╯       ",
        "            ·   ·   ✦            ",
    ],
    [
        "            ·   ✦   ·            ",
        "       ╭────◌───────◌────╮       ",
        "    ·  │   ╱    ·    ╲   │       ",
        "       ◌─────╱  ●  ╲─────◌  ·    ",
        "    ·  │   ╲    ·    ╱   │       ",
        "       ╰────◌───────◌────╯       ",
        "            ·   ·   ·            ",
    ],
];

const COMPACT_FRAMES: [[&str; 5]; FRAME_COUNT] = [
    [
        "          ·  ✦  ·          ",
        "     ╭────◌─────◌────╮     ",
        "  ·  ◌─────╲ ◉ ╱─────◌  ·  ",
        "     ╰────◌─────◌────╯     ",
        "          ·  ·  ·          ",
    ],
    [
        "          ·  ·  ✦          ",
        "     ╭────◌─────◌────╮     ",
        "     ◌─────╱ ◉ ╲─────◌  ·  ",
        "     ╰────◌─────◌────╯     ",
        "          ·  ·  ·          ",
    ],
    [
        "          ✦  ·  ·          ",
        "     ╭────◌─────◌────╮     ",
        "  ·  ◌─────╲ ● ╱─────◌  ·  ",
        "     ╰────◌─────◌────╯     ",
        "          ·  ·  ✦          ",
    ],
    [
        "          ·  ·  ·          ",
        "     ╭────◌───✦─◌────╮     ",
        "  ·  ◌─────╱ ◉ ╲─────◌  ·  ",
        "     ╰────◌─────◌────╯     ",
        "          ✦  ·  ·          ",
    ],
    [
        "          ·  ·  ·          ",
        "     ╭────◌─────◌────╮     ",
        "  ·  ◌─────╲ ◉ ╱─────◌  ·  ",
        "     ╰────◌─────◌────╯     ",
        "          ·  ·  ✦          ",
    ],
    [
        "          ·  ✦  ·          ",
        "     ╭────◌─────◌────╮     ",
        "     ◌─────╱ ● ╲─────◌  ·  ",
        "     ╰────◌─────◌────╯     ",
        "          ·  ·  ·          ",
    ],
];

const ASCII_CORE_FRAMES: [[&str; 7]; FRAME_COUNT] = [
    [
        "          .   *   .          ",
        "     .----o-------o----.     ",
        "  .  |   \\   .   /   |  .  ",
        "     o-----\\ (O) /-----o     ",
        "  .  |   /   .   \\   |  .  ",
        "     '----o-------o----'     ",
        "          .   .   .          ",
    ],
    [
        "          .   .   *          ",
        "     .----o-------o----.     ",
        "     |   /   .   \\   |  .  ",
        "  .  o-----/ (O) \\-----o     ",
        "     |   \\   .   /   |  .  ",
        "     '----o-------o----'     ",
        "          .   .   .          ",
    ],
    [
        "          *   .   .          ",
        "     .----o-------o----.     ",
        "     |   \\   .   /   |      ",
        "  .  o-----\\ (O) /-----o  .  ",
        "     |   /   .   \\   |      ",
        "     '----o-------o----'     ",
        "          .   .   *          ",
    ],
    [
        "          .   .   .          ",
        "     .----o---*---o----.     ",
        "     |   /   .   \\   |      ",
        "  .  o-----/ (O) \\-----o  .  ",
        "     |   \\   .   /   |      ",
        "     '----o-------o----'     ",
        "          *   .   .          ",
    ],
    [
        "          .   .   .          ",
        "     .----o-------o----.     ",
        "     |   \\   *   /   |      ",
        "  .  o-----\\ (O) /-----o  .  ",
        "     |   /   .   \\   |      ",
        "     '----o-------o----'     ",
        "          .   .   *          ",
    ],
    [
        "          .   *   .          ",
        "     .----o-------o----.     ",
        "  .  |   /   .   \\   |      ",
        "     o-----/ (O) \\-----o  .  ",
        "  .  |   \\   .   /   |      ",
        "     '----o-------o----'     ",
        "          .   .   .          ",
    ],
];

const ASCII_COMPACT_FRAMES: [[&str; 5]; FRAME_COUNT] = [
    [
        "        .  *  .        ",
        "   .----o-----o----.   ",
        " . o----- (O) -----o . ",
        "   '----o-----o----'   ",
        "        .  .  .        ",
    ],
    [
        "        .  .  *        ",
        "   .----o-----o----.   ",
        "   o----- (O) -----o . ",
        "   '----o-----o----'   ",
        "        .  .  .        ",
    ],
    [
        "        *  .  .        ",
        "   .----o-----o----.   ",
        " . o----- (O) -----o . ",
        "   '----o-----o----'   ",
        "        .  .  *        ",
    ],
    [
        "        .  .  .        ",
        "   .----o--*--o----.   ",
        " . o----- (O) -----o . ",
        "   '----o-----o----'   ",
        "        *  .  .        ",
    ],
    [
        "        .  .  .        ",
        "   .----o-----o----.   ",
        " . o----- (O) -----o . ",
        "   '----o-----o----'   ",
        "        .  .  *        ",
    ],
    [
        "        .  *  .        ",
        "   .----o-----o----.   ",
        "   o----- (O) -----o . ",
        "   '----o-----o----'   ",
        "        .  .  .        ",
    ],
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LogoSize {
    Compact,
    Full,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LogoPalette {
    Aurora,
    Violet,
    Mono,
}

impl LogoPalette {
    fn from_environment() -> Self {
        match std::env::var("BEECODE_LOGO_PALETTE")
            .or_else(|_| std::env::var("BEECODE_LOGO_PALETTE"))
            .ok()
            .as_deref()
            .map(str::trim)
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("violet") | Some("purple") => Self::Violet,
            Some("mono") | Some("monochrome") => Self::Mono,
            _ => Self::Aurora,
        }
    }

    fn target(self, position: f32, theme: &Theme) -> Color {
        match self {
            Self::Aurora if position < 0.42 => Color::Rgb(244, 185, 66),
            Self::Aurora if position < 0.75 => Color::Rgb(245, 158, 11),
            Self::Aurora => Color::Rgb(255, 209, 102),
            Self::Violet if position < 0.48 => Color::Rgb(245, 158, 11),
            Self::Violet => Color::Rgb(250, 204, 21),
            Self::Mono => theme.text_primary,
        }
    }
}

/// Reusable terminal startup animation settings. `BEECODE_LOGO_SPEED` accepts a
/// multiplier in 0.25..=4.0. The deprecated `BEECODE_LOGO_SPEED` remains supported.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LogoAnimation {
    palette: LogoPalette,
    speed: f32,
}

impl LogoAnimation {
    fn from_environment() -> Self {
        let speed = std::env::var("BEECODE_LOGO_SPEED")
            .or_else(|_| std::env::var("BEECODE_LOGO_SPEED"))
            .ok()
            .and_then(|v| v.trim().parse::<f32>().ok())
            .filter(|v| v.is_finite())
            .map(|v| v.clamp(0.25, 4.0))
            .unwrap_or(1.0);
        Self {
            palette: LogoPalette::from_environment(),
            speed,
        }
    }
    fn frame_index(self, secs: f32) -> usize {
        ((secs * ANIMATION_FPS * self.speed) as usize) % FRAME_COUNT
    }
    fn scaled_secs(self, secs: f32) -> f32 {
        secs * self.speed
    }
}

fn animation() -> &'static LogoAnimation {
    static CONFIG: OnceLock<LogoAnimation> = OnceLock::new();
    CONFIG.get_or_init(LogoAnimation::from_environment)
}

fn legacy_console() -> bool {
    crate::glyphs::is_legacy_windows_console()
}

fn pick_logo_for(window_height: u16) -> Option<LogoSize> {
    if window_height < SMALL_LOGO_MIN_HEIGHT {
        None
    } else if window_height < FULL_LOGO_MIN_HEIGHT {
        Some(LogoSize::Compact)
    } else {
        Some(LogoSize::Full)
    }
}

fn lines_for(size: LogoSize, frame: usize, ascii: bool) -> &'static [&'static str] {
    match (size, ascii) {
        (LogoSize::Full, false) => &CORE_FRAMES[frame],
        (LogoSize::Compact, false) => &COMPACT_FRAMES[frame],
        (LogoSize::Full, true) => &ASCII_CORE_FRAMES[frame],
        (LogoSize::Compact, true) => &ASCII_COMPACT_FRAMES[frame],
    }
}

fn line_count(size: LogoSize) -> u16 {
    match size {
        LogoSize::Compact => COMPACT_FRAMES[0].len() as u16,
        LogoSize::Full => CORE_FRAMES[0].len() as u16,
    }
}

fn visual_width(size: LogoSize, ascii: bool) -> u16 {
    lines_for(size, 0, ascii)
        .iter()
        .map(|line| unicode_width::UnicodeWidthStr::width(*line))
        .max()
        .unwrap_or(24) as u16
}

fn anim_phase_secs() -> f32 {
    static START: OnceLock<Instant> = OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_secs_f32()
}

/// Quantized redraw cadence prevents needless full-frame repaints.
pub fn shimmer_frame() -> u64 {
    (anim_phase_secs() * ANIMATION_FPS * animation().speed) as u64
}

fn glyph_strength(ch: char) -> f32 {
    match ch {
        '◉' | '●' | '✦' | '*' => 1.0,
        '◌' | 'o' | 'O' => 0.76,
        '╭' | '╮' | '╰' | '╯' | '─' | '│' | '╲' | '╱' | '-' | '|' | '/' | '\\' | '(' | ')' => {
            0.52
        }
        '·' | '.' => 0.32,
        _ => 0.0,
    }
}

fn render_into(area: Rect, buf: &mut Buffer, theme: &Theme, size: LogoSize) {
    let animation = *animation();
    let elapsed = anim_phase_secs();
    let secs = animation.scaled_secs(elapsed);
    let lines = lines_for(size, animation.frame_index(elapsed), legacy_console());
    let rows = lines.len().max(1) as f32;
    let cols = lines
        .iter()
        .map(|line| unicode_width::UnicodeWidthStr::width(*line))
        .max()
        .unwrap_or(1)
        .max(1) as f32;
    let logo_lines = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let mut spans = Vec::new();
            let mut run = String::new();
            let mut run_color = None;
            let mut width = 0usize;
            for ch in line.chars() {
                let position = (width as f32 + row as f32 * 0.45) / (cols + rows * 0.45);
                let pulse = 0.78 + 0.22 * (std::f32::consts::TAU * (secs / 2.8 + position)).sin();
                let strength = glyph_strength(ch);
                let color = if strength == 0.0 {
                    theme.bg_base
                } else {
                    let target = animation.palette.target(position, theme);
                    blend_color(theme.bg_base, target, (strength * pulse).clamp(0.0, 1.0))
                        .unwrap_or(theme.gray)
                };
                if run_color != Some(color) {
                    if let Some(previous) = run_color {
                        spans.push(Span::styled(
                            std::mem::take(&mut run),
                            Style::default().fg(previous),
                        ));
                    }
                    run_color = Some(color);
                }
                run.push(ch);
                width += unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            }
            if let Some(color) = run_color {
                spans.push(Span::styled(run, Style::default().fg(color)));
            }
            Line::from(spans).alignment(Alignment::Center)
        })
        .collect::<Vec<_>>();
    Paragraph::new(logo_lines).render(area, buf);
}

pub fn logo_line_count(window_height: u16) -> u16 {
    pick_logo_for(window_height).map_or(0, line_count)
}
pub fn logo_visual_width(window_height: u16) -> u16 {
    pick_logo_for(window_height).map_or(24, |size| visual_width(size, legacy_console()))
}
pub fn render_logo(area: Rect, buf: &mut Buffer, theme: &Theme, window_height: u16) {
    if let Some(size) = pick_logo_for(window_height) {
        render_into(area, buf, theme, size);
    }
}
pub fn full_logo_line_count() -> u16 {
    line_count(LogoSize::Full)
}
pub fn full_logo_visual_width() -> u16 {
    visual_width(LogoSize::Full, legacy_console())
}
pub fn render_full_logo(area: Rect, buf: &mut Buffer, theme: &Theme) {
    render_into(area, buf, theme, LogoSize::Full);
}
pub fn compact_logo_line_count() -> u16 {
    line_count(LogoSize::Compact)
}
pub fn render_compact_logo(area: Rect, buf: &mut Buffer, theme: &Theme) {
    render_into(area, buf, theme, LogoSize::Compact);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_selection_preserves_existing_height_contract() {
        assert_eq!(pick_logo_for(SMALL_LOGO_MIN_HEIGHT - 1), None);
        assert_eq!(
            pick_logo_for(SMALL_LOGO_MIN_HEIGHT),
            Some(LogoSize::Compact)
        );
        assert_eq!(
            pick_logo_for(FULL_LOGO_MIN_HEIGHT - 1),
            Some(LogoSize::Compact)
        );
        assert_eq!(pick_logo_for(FULL_LOGO_MIN_HEIGHT), Some(LogoSize::Full));
    }

    #[test]
    fn animation_has_six_consistent_frames() {
        assert_eq!(CORE_FRAMES.len(), FRAME_COUNT);
        assert_eq!(COMPACT_FRAMES.len(), FRAME_COUNT);
        assert!(CORE_FRAMES.iter().all(|frame| frame.len() == 7));
        assert!(COMPACT_FRAMES.iter().all(|frame| frame.len() == 5));
    }

    #[test]
    fn ascii_fallback_is_ascii_only() {
        for frame in ASCII_CORE_FRAMES {
            for line in frame {
                assert!(line.is_ascii(), "fallback line must be ASCII: {line:?}");
            }
        }
        for frame in ASCII_COMPACT_FRAMES {
            for line in frame {
                assert!(line.is_ascii(), "fallback line must be ASCII: {line:?}");
            }
        }
    }

    #[test]
    fn animation_frame_cycles_and_speed_is_bounded() {
        let standard = LogoAnimation {
            palette: LogoPalette::Aurora,
            speed: 1.0,
        };
        assert_eq!(standard.frame_index(0.0), 0);
        assert_eq!(standard.frame_index(FRAME_COUNT as f32 / ANIMATION_FPS), 0);
        let slow = LogoAnimation {
            palette: LogoPalette::Mono,
            speed: 0.25,
        };
        let fast = LogoAnimation {
            palette: LogoPalette::Violet,
            speed: 4.0,
        };
        assert!(fast.frame_index(0.15) > slow.frame_index(0.15));
    }

    #[test]
    fn all_frames_fit_their_reserved_width() {
        let full_width = visual_width(LogoSize::Full, false) as usize;
        let compact_width = visual_width(LogoSize::Compact, false) as usize;
        for frame in CORE_FRAMES {
            assert!(
                frame
                    .iter()
                    .all(|line| unicode_width::UnicodeWidthStr::width(*line) <= full_width)
            );
        }
        for frame in COMPACT_FRAMES {
            assert!(
                frame
                    .iter()
                    .all(|line| unicode_width::UnicodeWidthStr::width(*line) <= compact_width)
            );
        }
    }
}
