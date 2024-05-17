use gpui::{
    actions, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView, IntoElement, Render,
    View, ViewContext, VisualContext, WeakView,
};
use theme::ActiveTheme;
use ui::{
    h_flex, v_flex, Color, InteractiveElement, Label, LabelCommon, ParentElement, Styled,
    WindowContext,
};
use workspace::{
    item::{Item, ItemEvent, TabContentParams},
    Workspace,
};

actions!(tungsten, [Patch]);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(move |workspace: &mut Workspace, cx| {
        workspace.register_action(move |workspace, _: &Patch, cx| {
            let existing = workspace
                .active_pane()
                .read(cx)
                .items()
                .find_map(|item| item.downcast::<PatchView>());

            if let Some(existing) = existing {
                workspace.activate_item(&existing, cx);
            } else {
                let patch_view = PatchView::new(workspace, cx);
                workspace.add_item_to_active_pane(Box::new(patch_view), None, cx)
            }
        });
    })
    .detach();
}

pub struct PatchView {
    workspace: WeakView<Workspace>,
    focus_handle: FocusHandle,
}

impl PatchView {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        cx.new_view(|cx: &mut ViewContext<Self>| {
            let focus_handle = cx.focus_handle();

            let this = Self {
                workspace: workspace.weak_handle(),
                focus_handle,
            };

            this
        })
    }
}

impl Render for PatchView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .bg(cx.theme().colors().tab_active_background)
            .track_focus(&self.focus_handle)
            .child(
                v_flex()
                    .w_96()
                    .gap_4()
                    .mx_auto()
                    .child(h_flex().justify_center().child(Label::new("Patch List"))),
            )
    }
}

impl EventEmitter<ItemEvent> for PatchView {}

impl FocusableView for PatchView {
    fn focus_handle(&self, _: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for PatchView {
    type Event = ItemEvent;

    fn tab_content(&self, params: TabContentParams, _: &WindowContext) -> AnyElement {
        Label::new("Patches")
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }
}
