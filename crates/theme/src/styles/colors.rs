use gpui::{rgb, Hsla, Refineable, Rgba};

#[derive(Refineable, Clone, Debug)]
#[refineable(Debug)]
pub struct ThemeColors {
    /// Border color. Used for most borders, is usually a high contrast color.
    pub border: Rgba,
    /// Background Color. Used for the background of an element that should have a different background than the surface it's on.
    pub background: Rgba,
    /// Text Color. Default text color used for most text.
    pub text: Rgba,

    pub title_bar_background: Rgba,
}

impl Default for ThemeColors {
    fn default() -> Self {
        ThemeColors {
            border: rgb(0x3b3a37),
            background: rgb(0x111110),
            text: rgb(0xeeeeec),
            title_bar_background: rgb(0x191918),
        }
    }
}

#[derive(Refineable, Clone)]
pub struct ThemeStyles {
    #[refineable]
    pub colors: ThemeColors,
}
