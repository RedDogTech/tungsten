use gpui::{Hsla, WindowContext};
use theme::ActiveTheme;

/// Sets a color that has a consistent meaning across all themes.
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    Default,
    Muted,
    Hidden,
    Accent,
    Error,
    Warning,
    Disabled,
    Selected,
}

impl Color {
    pub fn color(&self, cx: &WindowContext) -> Hsla {
        match self {
            Color::Default => cx.theme().colors().text,
            Color::Muted => cx.theme().colors().text_muted,
            Color::Hidden => cx.theme().colors().hidden,
            Color::Accent => cx.theme().colors().text_accent,
            Color::Error => cx.theme().colors().error,
            Color::Warning => cx.theme().colors().warning,
            Color::Disabled => cx.theme().colors().text_disabled,
            Color::Selected => cx.theme().colors().text_accent,
        }
    }
}
