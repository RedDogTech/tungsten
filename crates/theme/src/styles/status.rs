use gpui::{rgb, Hsla, Refineable};

#[derive(Refineable, Clone, Debug)]
pub struct StatusColors {
    pub info_background: Hsla,
    pub info_border: Hsla,
    pub error_background: Hsla,
    pub error_border: Hsla,
    pub warning_background: Hsla,
    pub warning_border: Hsla,
}

impl Default for StatusColors {
    fn default() -> Self {
        StatusColors {
            info_background: rgb(0x18243d).into(),
            info_border: rgb(0x293b5b).into(),
            error_background: rgb(0x301b1b).into(),
            error_border: rgb(0x4c2b2c).into(),
            warning_background: rgb(0x41321d).into(),
            warning_border: rgb(0x5d4c2f).into(),
        }
    }
}
