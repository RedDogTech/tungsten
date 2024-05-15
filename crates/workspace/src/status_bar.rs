use std::any::TypeId;

use crate::{item::ItemHandle, pane::Pane};
use gpui::{AnyView, Render, View};
use theme::ActiveTheme;
use ui::{h_flex, IntoElement, ParentElement, Spacing, Styled, ViewContext, WindowContext};

trait StatusItemViewHandle: Send {
    fn to_any(&self) -> AnyView;
    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    );
    fn item_type(&self) -> TypeId;
}

pub struct StatusBar {
    left_items: Vec<Box<dyn StatusItemViewHandle>>,
    right_items: Vec<Box<dyn StatusItemViewHandle>>,
    active_pane: View<Pane>,
}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .gap(Spacing::Large.rems(cx))
            .py(Spacing::Small.rems(cx))
            .px(Spacing::Large.rems(cx))
            // .h_8()
            .bg(cx.theme().colors().status_bar_background)
            .child(self.render_left_tools(cx))
            .child(self.render_right_tools(cx))
    }
}

impl StatusBar {
    pub fn new(active_pane: &View<Pane>, cx: &mut ViewContext<Self>) -> Self {
        let mut this = Self {
            left_items: Default::default(),
            right_items: Default::default(),
            active_pane: active_pane.clone(),
        };
        this
    }

    fn render_left_tools(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .gap(Spacing::Large.rems(cx))
            .overflow_x_hidden()
            .children(self.left_items.iter().map(|item| item.to_any()))
    }

    fn render_right_tools(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .gap(Spacing::Large.rems(cx))
            .children(self.right_items.iter().rev().map(|item| item.to_any()))
    }
}
