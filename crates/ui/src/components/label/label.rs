use gpui::{IntoElement, ParentElement, RenderOnce, SharedString, WindowContext};

use crate::{Color, LabelCommon, LabelLike, LabelSize, LineHeightStyle};

#[derive(IntoElement)]
pub struct Label {
    base: LabelLike,
    label: SharedString,
    single_line: bool,
}

impl Label {
    /// Creates a new [`Label`] with the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!");
    /// ```
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: LabelLike::new(),
            label: label.into(),
            single_line: false,
        }
    }

    /// Make the label display in a single line mode
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").single_line(true);
    /// ```
    pub fn single_line(mut self) -> Self {
        self.single_line = true;
        self
    }
}
impl LabelCommon for Label {
    /// Sets the size of the label using a [`LabelSize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").size(LabelSize::Small);
    /// ```
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    /// Sets the line height style of the label using a [`LineHeightStyle`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").line_height_style(LineHeightStyle::UiLabel);
    /// ```
    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self
    }

    /// Sets the color of the label using a [`Color`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").color(Color::Accent);
    /// ```
    fn color(mut self, color: Color) -> Self {
        self.base = self.base.color(color);
        self
    }

    /// Sets the strikethrough property of the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").strikethrough(true);
    /// ```
    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.base = self.base.strikethrough(strikethrough);
        self
    }

    /// Sets the italic property of the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").italic(true);
    /// ```
    fn italic(mut self, italic: bool) -> Self {
        self.base = self.base.italic(italic);
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let target_label = if self.single_line {
            SharedString::from(self.label.replace('\n', "␤"))
        } else {
            self.label
        };
        self.base.child(target_label)
    }
}
