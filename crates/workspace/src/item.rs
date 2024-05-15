use gpui::{AnyElement, AnyView, Entity, EntityId, EventEmitter, FocusHandle, FocusableView, View};
use ui::{Element, WindowContext};

pub trait Item: FocusableView + EventEmitter<Self::Event> {
    type Event;

    fn tab_content(&self, _params: TabContentParams, _cx: &WindowContext) -> AnyElement {
        gpui::Empty.into_any()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ItemEvent {}

pub trait ItemHandle: 'static + Send {
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn to_any(&self) -> AnyView;
    fn item_id(&self) -> EntityId;
    fn focus_handle(&self, cx: &WindowContext) -> FocusHandle;
    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement;
}

impl dyn ItemHandle {
    pub fn downcast<V: 'static>(&self) -> Option<View<V>> {
        self.to_any().downcast().ok()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TabContentParams {
    pub detail: Option<usize>,
    pub selected: bool,
    pub preview: bool,
}

impl<T: Item> ItemHandle for View<T> {
    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn item_id(&self) -> EntityId {
        self.entity_id()
    }

    fn focus_handle(&self, cx: &WindowContext) -> FocusHandle {
        self.focus_handle(cx)
    }

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        self.read(cx).tab_content(params, cx)
    }
}

impl Clone for Box<dyn ItemHandle> {
    fn clone(&self) -> Box<dyn ItemHandle> {
        self.boxed_clone()
    }
}
