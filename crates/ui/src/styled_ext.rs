use gpui::{hsla, px, Styled, WindowContext};

/// Extends [`gpui::Styled`] with Zed-specific styling methods.
pub trait StyledExt: Styled + Sized {
    /// Horizontally stacks elements.
    ///
    /// Sets `flex()`, `flex_row()`, `items_center()`
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }
}

impl<E: Styled> StyledExt for E {}
