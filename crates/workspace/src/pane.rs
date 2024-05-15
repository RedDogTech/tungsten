use std::cmp;

use gpui::*;
use gpui::{FocusHandle, KeyContext, Render, WeakView};
use serde::Deserialize;
use ui::{
    v_flex, Color, FluentBuilder, InteractiveElement, IntoElement, Label, LabelCommon, Selectable,
    StyledExt, Tab, TabBar, TabPosition, ViewContext,
};

use crate::item::TabContentParams;
use crate::{item::ItemHandle, Workspace};

impl_actions!(pane, [ActivateItem]);

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct ActivateItem(pub usize);

pub struct Pane {
    focus_handle: FocusHandle,
    items: Vec<Box<dyn ItemHandle>>,
    preview_item_id: Option<EntityId>,
    active_item_index: usize,
    pub(crate) workspace: WeakView<Workspace>,
}

impl Pane {
    pub fn new(workspace: WeakView<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            focus_handle,
            items: Vec::new(),
            active_item_index: 0,
            preview_item_id: None,
            workspace,
        }
    }

    pub fn active_item(&self) -> Option<Box<dyn ItemHandle>> {
        self.items.get(self.active_item_index).cloned()
    }

    pub fn items(&self) -> impl DoubleEndedIterator<Item = &Box<dyn ItemHandle>> {
        self.items.iter()
    }

    pub fn index_for_item(&self, item: &dyn ItemHandle) -> Option<usize> {
        self.items
            .iter()
            .position(|i| i.item_id() == item.item_id())
    }

    pub fn focus(&mut self, cx: &mut ViewContext<Pane>) {
        cx.focus(&self.focus_handle);
    }

    pub fn focus_active_item(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(active_item) = self.active_item() {
            let focus_handle = active_item.focus_handle(cx);
            cx.focus(&focus_handle);
        }
    }

    pub fn add_item(
        &mut self,
        item: Box<dyn ItemHandle>,
        activate_pane: bool,
        focus_item: bool,
        destination_index: Option<usize>,
        cx: &mut ViewContext<Self>,
    ) {
        let insertion_index = {
            cmp::min(
                if let Some(destination_index) = destination_index {
                    destination_index
                } else {
                    self.active_item_index + 1
                },
                self.items.len(),
            )
        };

        self.items.insert(insertion_index, item.clone());
        self.activate_item(insertion_index, activate_pane, focus_item, cx);
    }

    pub fn activate_item(
        &mut self,
        index: usize,
        activate_pane: bool,
        focus_item: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if index < self.items.len() {
            if focus_item {
                self.focus_active_item(cx);
            }
            cx.notify();
        }
    }

    fn render_tab(
        &self,
        ix: usize,
        item: &Box<dyn ItemHandle>,
        detail: usize,
        cx: &mut ViewContext<'_, Pane>,
    ) -> impl IntoElement {
        let is_active = ix == self.active_item_index;
        let is_preview = self
            .preview_item_id
            .map(|id| id == item.item_id())
            .unwrap_or(false);

        let is_first_item = ix == 0;
        let is_last_item = ix == self.items.len() - 1;
        let position_relative_to_active_item = ix.cmp(&self.active_item_index);

        let label = item.tab_content(
            TabContentParams {
                detail: Some(detail),
                selected: is_active,
                preview: is_preview,
            },
            cx,
        );

        Tab::new(ix)
            .position(if is_first_item {
                TabPosition::First
            } else if is_last_item {
                TabPosition::Last
            } else {
                TabPosition::Middle(position_relative_to_active_item)
            })
            .selected(is_active)
            .child(label)
    }

    fn render_tab_bar(&mut self, cx: &mut ViewContext<'_, Pane>) -> impl IntoElement {
        TabBar::new("tab_bar").children(
            self.items
                .iter()
                .enumerate()
                .zip(tab_details(&self.items, cx))
                .map(|((ix, item), detail)| self.render_tab(ix, item, detail, cx)),
        )
    }
}

pub fn tab_details(items: &Vec<Box<dyn ItemHandle>>, cx: &AppContext) -> Vec<usize> {
    let tab_details = items.iter().map(|_| 0).collect::<Vec<_>>();

    tab_details
}

impl FocusableView for Pane {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Pane {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("Pane");
        if self.active_item().is_none() {
            key_context.add("EmptyPane");
        }

        v_flex()
            .key_context(key_context)
            .track_focus(&self.focus_handle)
            .size_full()
            .flex_none()
            .overflow_hidden()
            .when(self.active_item().is_some(), |pane| {
                pane.child(self.render_tab_bar(cx))
            })
            .child({
                div().flex_1().relative().group("").map(|div| {
                    if let Some(item) = self.active_item() {
                        div.v_flex().child(item.to_any())
                    } else {
                        let placeholder = div.h_flex().size_full().justify_center();
                        placeholder.child(
                            Label::new("Open a file or project to get started.")
                                .color(Color::Muted),
                        )
                    }
                })
            })
    }
}
