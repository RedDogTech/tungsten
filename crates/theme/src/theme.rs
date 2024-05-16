mod settings;
mod styles;
use ::settings::Settings;
use gpui::{AppContext, WindowBackgroundAppearance};
pub use settings::*;
use std::sync::Arc;
pub use styles::*;

pub fn init(cx: &mut AppContext) {
    ThemeSettings::register(cx);
}

pub trait ActiveTheme {
    fn theme(&self) -> Arc<Theme>;
}

impl ActiveTheme for AppContext {
    fn theme(&self) -> Arc<Theme> {
        Arc::new(Theme::default())
    }
}

#[derive(Clone)]
pub struct Theme {
    pub styles: ThemeStyles,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            styles: ThemeStyles {
                colors: ThemeColors::default(),
                window_background_appearance: WindowBackgroundAppearance::Opaque,
                status: StatusColors::default(),
            },
        }
    }
}

impl Theme {
    /// Returns the [`ThemeColors`] for the theme.
    #[inline(always)]
    pub fn colors(&self) -> &ThemeColors {
        &self.styles.colors
    }

    /// Returns the [`StatusColors`] for the theme.
    #[inline(always)]
    pub fn status(&self) -> &StatusColors {
        &self.styles.status
    }

    #[inline(always)]
    pub fn window_background_appearance(&self) -> WindowBackgroundAppearance {
        self.styles.window_background_appearance
    }
}
