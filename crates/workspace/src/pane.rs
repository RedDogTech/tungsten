use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{cmp, mem};

use gpui::*;
use gpui::{FocusHandle, KeyContext, Render, WeakView};
use serde::Deserialize;
use ui::{
    v_flex, ButtonCommon, ButtonSize, Clickable, Color, FluentBuilder, IconButton, IconButtonShape,
    IconName, IconSize, InteractiveElement, IntoElement, Label, LabelCommon, Selectable, StyledExt,
    Tab, TabBar, TabPosition, ViewContext,
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
    activation_history: Vec<ActivationHistoryEntry>,
    next_activation_timestamp: Arc<AtomicUsize>,
}

pub struct ActivationHistoryEntry {
    pub entity_id: EntityId,
    pub timestamp: usize,
}

impl Pane {
    pub fn new(
        workspace: WeakView<Workspace>,
        next_timestamp: Arc<AtomicUsize>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            focus_handle,
            activation_history: Vec::new(),
            next_activation_timestamp: next_timestamp.clone(),
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
            let prev_active_item_ix = mem::replace(&mut self.active_item_index, index);
            if focus_item {
                self.focus_active_item(cx);
            }

            if let Some(newly_active_item) = self.items.get(index) {
                self.activation_history
                    .retain(|entry| entry.entity_id != newly_active_item.item_id());
                self.activation_history.push(ActivationHistoryEntry {
                    entity_id: newly_active_item.item_id(),
                    timestamp: self
                        .next_activation_timestamp
                        .fetch_add(1, Ordering::SeqCst),
                });
            }

            cx.notify();
        }
    }

    pub fn active_item_index(&self) -> usize {
        self.active_item_index
    }

    pub fn close_item_by_id(
        &mut self,
        item_id_to_close: EntityId,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.close_items(cx, move |view_id| view_id == item_id_to_close)
    }

    pub fn close_items(
        &mut self,
        cx: &mut ViewContext<Pane>,
        should_close: impl Fn(EntityId) -> bool,
    ) -> Task<Result<()>> {
        let workspace = self.workspace.clone();
        let mut items_to_close = Vec::new();

        for item in &self.items {
            if should_close(item.item_id()) {
                items_to_close.push(item.boxed_clone());
            }
        }

        cx.spawn(|pane, mut cx| async move {
            for item in items_to_close.clone() {
                // Find the item's current index and its set of project item models. Avoid
                // storing these in advance, in case they have changed since this task
                // was started.
                let item_ix = pane.update(&mut cx, |pane, cx| (pane.index_for_item(&*item)))?;
                let item_ix = if let Some(ix) = item_ix {
                    ix
                } else {
                    continue;
                };

                // Remove the item from the pane.
                pane.update(&mut cx, |pane, cx| {
                    if let Some(item_ix) = pane
                        .items
                        .iter()
                        .position(|i| i.item_id() == item.item_id())
                    {
                        pane.remove_item(item_ix, false, true, cx);
                    }
                })
                .ok();
            }

            pane.update(&mut cx, |_, cx| cx.notify()).ok();
            Ok(())
        })
    }

    pub fn has_focus(&self, cx: &WindowContext) -> bool {
        // We not only check whether our focus handle contains focus, but also
        // whether the active_item might have focus, because we might have just activated an item
        // but that hasn't rendered yet.
        // So before the next render, we might have transferred focus
        // to the item and `focus_handle.contains_focus` returns false because the `active_item`
        // is not hooked up to us in the dispatch tree.
        self.focus_handle.contains_focused(cx)
            || self
                .active_item()
                .map_or(false, |item| item.focus_handle(cx).contains_focused(cx))
    }

    pub fn remove_item(
        &mut self,
        item_index: usize,
        activate_pane: bool,
        close_pane_if_empty: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.activation_history
            .retain(|entry| entry.entity_id != self.items[item_index].item_id());

        if item_index == self.active_item_index {
            let index_to_activate = self
                .activation_history
                .pop()
                .and_then(|last_activated_item| {
                    self.items.iter().enumerate().find_map(|(index, item)| {
                        (item.item_id() == last_activated_item.entity_id).then_some(index)
                    })
                })
                // We didn't have a valid activation history entry, so fallback
                // to activating the item to the left
                .unwrap_or_else(|| item_index.min(self.items.len()).saturating_sub(1));

            let should_activate = activate_pane || self.has_focus(cx);
            if self.items.len() == 1 && should_activate {
                self.focus_handle.focus(cx);
            } else {
                self.activate_item(index_to_activate, should_activate, should_activate, cx);
            }
        }

        self.items.remove(item_index);

        // cx.emit(Event::RemoveItem {
        //     item_id: item.item_id(),
        // });

        if item_index < self.active_item_index {
            self.active_item_index -= 1;
        }

        cx.notify();
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

        let item_id = item.item_id();

        Tab::new(ix)
            .position(if is_first_item {
                TabPosition::First
            } else if is_last_item {
                TabPosition::Last
            } else {
                TabPosition::Middle(position_relative_to_active_item)
            })
            .close_side(ui::TabCloseSide::End)
            .selected(is_active)
            .on_click(
                cx.listener(move |pane: &mut Self, _, cx| pane.activate_item(ix, true, true, cx)),
            )
            .end_slot(
                IconButton::new("close tab", IconName::Close)
                    .shape(IconButtonShape::Square)
                    .icon_color(Color::Muted)
                    .size(ButtonSize::None)
                    .icon_size(IconSize::XSmall)
                    .on_click(cx.listener(move |pane, _, cx| {
                        pane.close_item_by_id(item_id, cx).detach_and_log_err(cx);
                    })),
            )
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
