use std::fmt;

use gpui_kit::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

impl fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
        };
        write!(f, "{name}")
    }
}

/// Every colour used by the UI, named by role rather than by value.
///
/// Stored as a gpui *global*: set it once with `cx.set_global(Theme::light())`,
/// read it anywhere with `Theme::get(cx)`. Switching mode is just setting a
/// different `Theme` and re-rendering.
#[derive(Debug, Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    /// Window background, behind everything.
    pub background: Hsla,
    /// Borders of the board, cells, buttons and tooltips.
    pub border: Hsla,
    /// Default text colour.
    pub text: Hsla,
    /// Background of floating surfaces like tooltips.
    pub surface: Hsla,

    pub button_bg: Hsla,
    pub button_hover: Hsla,

    pub cell_hidden: Hsla,
    pub cell_revealed: Hsla,
    /// Overlay drawn on top of a cell while it's hovered.
    pub cell_hover: Hsla,

    /// Colours for 1 to 8 neighbouring mines: `numbers[0]` is the colour of a 1.
    numbers: [Hsla; 8],
}

impl Global for Theme {}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            background: white(),
            border: rgb(0x909090).into(),
            text: black().opacity(0.74),
            surface: white(),

            button_bg: black().opacity(0.05),
            button_hover: black().opacity(0.07),

            cell_hidden: rgb(0xe7e7e7).into(),
            cell_revealed: rgb(0xc5c5c5).into(),
            cell_hover: white().opacity(0.2),

            numbers: [
                rgb(0x2563eb).into(), // blue
                rgb(0x16a34a).into(), // green
                rgb(0xdc2626).into(), // red
                rgb(0x7c3aed).into(), // purple
                rgb(0xb45309).into(), // brown
                rgb(0x0891b2).into(), // teal
                rgb(0x111111).into(), // near-black
                rgb(0x6b7280).into(), // grey
            ],
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            background: rgb(0x1e1e1e).into(),
            border: rgb(0x5a5a5a).into(),
            text: white().opacity(0.74),
            surface: rgb(0x2a2a2a).into(),

            button_bg: white().opacity(0.06),
            button_hover: white().opacity(0.1),

            cell_hidden: rgb(0x3c3c3c).into(),
            cell_revealed: rgb(0x262626).into(),
            cell_hover: white().opacity(0.08),

            // Lighter shades, so they stay readable on a dark background.
            numbers: [
                rgb(0x60a5fa).into(), // blue
                rgb(0x4ade80).into(), // green
                rgb(0xf87171).into(), // red
                rgb(0xa78bfa).into(), // purple
                rgb(0xfbbf24).into(), // amber
                rgb(0x22d3ee).into(), // teal
                rgb(0xe5e5e5).into(), // near-white
                rgb(0x9ca3af).into(), // grey
            ],
        }
    }

    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }

    /// The current theme. Works with any context (`App`, `Context<T>`).
    pub fn get(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    /// Colour of a revealed cell showing `n` neighbouring mines (1 to 8).
    pub fn number(&self, n: usize) -> Hsla {
        self.numbers[n.clamp(1, 8) - 1]
    }
}
