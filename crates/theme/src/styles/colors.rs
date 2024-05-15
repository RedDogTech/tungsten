use gpui::{rgb, Hsla, Refineable, Rgba, WindowBackgroundAppearance};

#[derive(Refineable, Clone, Debug)]
#[refineable(Debug)]
pub struct ThemeColors {
    pub border: Hsla,
    pub background: Hsla,
    pub text: Hsla,
    pub title_bar_background: Hsla,
    pub text_muted: Hsla,
    pub hidden: Hsla,
    pub elevated_surface_background: Hsla,
    pub tab_inactive_background: Hsla,
    pub ghost_element_hover: Hsla,
    pub ghost_element_active: Hsla,
    pub element_hover: Hsla,
    pub element_active: Hsla,
    pub tab_active_background: Hsla,
    pub tab_bar_background: Hsla,
    pub status_bar_background: Hsla,
}

impl Default for ThemeColors {
    fn default() -> Self {
        ThemeColors {
            border: rgb(0x464b57).into(),
            background: rgb(0x3b414d).into(),
            text: rgb(0xc8ccd4).into(),
            text_muted: rgb(0x838994).into(),
            title_bar_background: rgb(0x3b414d).into(),
            hidden: rgb(0x555a63).into(),
            elevated_surface_background: rgb(0x2f343e).into(),
            tab_inactive_background: rgb(0x2f343e).into(),
            ghost_element_hover: rgb(0x363c46).into(),
            ghost_element_active: rgb(0x454a56).into(),
            element_hover: rgb(0x363c46).into(),
            element_active: rgb(0x454a56).into(),
            tab_active_background: rgb(0x282c33).into(),
            tab_bar_background: rgb(0x2f343e).into(),
            status_bar_background: rgb(0x3b414d).into(),
        }
    }
}

#[derive(Refineable, Clone)]
pub struct ThemeStyles {
    #[refineable]
    pub colors: ThemeColors,

    pub window_background_appearance: WindowBackgroundAppearance,
}
