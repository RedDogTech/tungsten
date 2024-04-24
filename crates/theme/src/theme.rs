mod styles;

use std::sync::Arc;

use gpui::{AppContext, WindowBackgroundAppearance};
pub use styles::*;

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

    #[inline(always)]
    pub fn window_background_appearance(&self) -> WindowBackgroundAppearance {
        self.styles.window_background_appearance
    }
}
