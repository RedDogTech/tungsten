use gpui::{rgb, Refineable, Rgba, WindowBackgroundAppearance};

#[derive(Refineable, Clone, Debug)]
#[refineable(Debug)]
pub struct ThemeColors {
    pub border: Rgba,
    pub background: Rgba,
    pub text: Rgba,
    pub title_bar_background: Rgba,
}

impl Default for ThemeColors {
    fn default() -> Self {
        ThemeColors {
            border: rgb(0x464b57),
            background: rgb(0x3b414d),
            text: rgb(0xc8ccd4),
            title_bar_background: rgb(0x3b414d),
        }
    }
}

#[derive(Refineable, Clone)]
pub struct ThemeStyles {
    #[refineable]
    pub colors: ThemeColors,

    pub window_background_appearance: WindowBackgroundAppearance,
}
