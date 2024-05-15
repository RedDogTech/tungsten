use gpui::{Hsla, WindowContext};
use theme::ActiveTheme;

/// Sets a color that has a consistent meaning across all themes.
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    Default,
    Muted,
    Hidden,
}

impl Color {
    pub fn color(&self, cx: &WindowContext) -> Hsla {
        match self {
            Color::Default => cx.theme().colors().text.into(),
            Color::Muted => cx.theme().colors().text_muted.into(),
            Color::Hidden => cx.theme().colors().hidden.into(),
        }
    }
}
