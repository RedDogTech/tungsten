use std::any::TypeId;

use crate::{item::ItemHandle, pane::Pane};
use gpui::{AnyView, Render, View};
use theme::ActiveTheme;
use ui::{h_flex, IntoElement, ParentElement, Spacing, Styled, ViewContext, WindowContext};

pub trait StatusItemView: Render {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn crate::ItemHandle>,
        cx: &mut ViewContext<Self>,
    );
}

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

    pub fn add_left_item<T>(&mut self, item: View<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), cx);

        self.left_items.push(Box::new(item));
        cx.notify();
    }

    pub fn add_right_item<T>(&mut self, item: View<T>, cx: &mut ViewContext<Self>)
    where
        T: 'static + StatusItemView,
    {
        let active_pane_item = self.active_pane.read(cx).active_item();
        item.set_active_pane_item(active_pane_item.as_deref(), cx);

        self.right_items.push(Box::new(item));
        cx.notify();
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

impl<T: StatusItemView> StatusItemViewHandle for View<T> {
    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn set_active_pane_item(
        &self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut WindowContext,
    ) {
        self.update(cx, |this, cx| {
            this.set_active_pane_item(active_pane_item, cx)
        });
    }

    fn item_type(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl From<&dyn StatusItemViewHandle> for AnyView {
    fn from(val: &dyn StatusItemViewHandle) -> Self {
        val.to_any().clone()
    }
}
