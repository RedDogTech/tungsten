use gpui::{
    div, impl_actions, Action, AnyView, AppContext, FocusHandle, FocusableView, Global,
    InteractiveElement, IntoElement, KeyContext, ParentElement, Render, Styled, View, ViewContext,
    VisualContext, WeakView, WindowContext, WindowOptions,
};
use item::ItemHandle;
use pane::Pane;
use pane_group::PaneGroup;
use serde::Deserialize;
use std::sync::{Arc, Weak};
use theme::ActiveTheme;
use ui::{h_flex, Div, TitleBar};
use uuid::Uuid;

pub mod item;
pub mod pane;
pub mod pane_group;
mod status_bar;
use status_bar::StatusBar;

impl_actions!(workspace, [ActivatePane]);

#[derive(Clone, Deserialize, PartialEq)]
pub struct ActivatePane(pub usize);

pub struct AppState {
    pub build_window_options: fn(Option<Uuid>, &mut AppContext) -> WindowOptions,
}

struct GlobalAppState(Weak<AppState>);

impl Global for GlobalAppState {}

impl AppState {
    pub fn global(cx: &AppContext) -> Weak<Self> {
        cx.global::<GlobalAppState>().0.clone()
    }

    pub fn try_global(cx: &AppContext) -> Option<Weak<Self>> {
        cx.try_global::<GlobalAppState>()
            .map(|state| state.0.clone())
    }

    pub fn set_global(state: Weak<AppState>, cx: &mut AppContext) {
        cx.set_global(GlobalAppState(state));
    }
}

pub struct Workspace {
    workspace_actions: Vec<Box<dyn Fn(Div, &mut ViewContext<Self>) -> Div>>,
    weak_self: WeakView<Self>,
    active_pane: View<Pane>,
    panes: Vec<View<Pane>>,
    center: PaneGroup,
    titlebar_item: Option<AnyView>,
    app_state: Arc<AppState>,
    status_bar: View<StatusBar>,
}

pub fn new(app_state: Arc<AppState>, cx: &mut AppContext) {
    cx.spawn(|cx| async move {
        let mut options = cx
            .update(|cx| (app_state.build_window_options)(None, cx))
            .unwrap();

        cx.open_window(options, {
            let app_state = app_state.clone();
            move |cx| cx.new_view(|cx| Workspace::new(app_state, cx))
        });
    })
    .detach();
}

impl Workspace {
    pub fn new(app_state: Arc<AppState>, cx: &mut ViewContext<Self>) -> Self {
        let weak_handle = cx.view().downgrade();

        cx.defer(|this, cx| {
            this.update_window_title(cx);
        });

        let center_pane = cx.new_view(|cx| Pane::new(weak_handle.clone(), cx));

        let status_bar = cx.new_view(|cx| {
            let mut status_bar = StatusBar::new(&center_pane.clone(), cx);
            status_bar
        });

        Workspace {
            workspace_actions: Default::default(),
            weak_self: weak_handle.clone(),
            active_pane: center_pane.clone(),
            panes: vec![center_pane.clone()],
            center: PaneGroup::new(center_pane.clone()),
            titlebar_item: None,
            app_state,
            status_bar,
        }
    }

    pub fn active_pane(&self) -> &View<Pane> {
        &self.active_pane
    }

    pub fn activate_item(&mut self, item: &dyn ItemHandle, cx: &mut WindowContext) -> bool {
        let result = self.panes.iter().find_map(|pane| {
            pane.read(cx)
                .index_for_item(item)
                .map(|ix| (pane.clone(), ix))
        });
        if let Some((pane, ix)) = result {
            pane.update(cx, |pane, cx| pane.activate_item(ix, true, true, cx));
            true
        } else {
            false
        }
    }

    pub fn add_item_to_active_pane(
        &mut self,
        item: Box<dyn ItemHandle>,
        destination_index: Option<usize>,
        cx: &mut WindowContext,
    ) {
        self.add_item(self.active_pane.clone(), item, destination_index, cx)
    }

    pub fn add_item(
        &mut self,
        pane: View<Pane>,
        item: Box<dyn ItemHandle>,
        destination_index: Option<usize>,
        cx: &mut WindowContext,
    ) {
        pane.update(cx, |pane, cx| {
            pane.add_item(item, true, true, destination_index, cx)
        });
    }

    fn update_window_title(&mut self, cx: &mut WindowContext) {
        let mut title = String::new();
        title = "empty project".to_string();
        cx.set_window_title(&title);
    }

    fn titlebar_item(&mut self) -> impl IntoElement {
        TitleBar::new("collab-titlebar")
    }

    pub fn weak_handle(&self) -> WeakView<Self> {
        self.weak_self.clone()
    }

    pub fn register_action<A: Action>(
        &mut self,
        callback: impl Fn(&mut Self, &A, &mut ViewContext<Self>) + 'static,
    ) -> &mut Self {
        let callback = Arc::new(callback);

        self.workspace_actions.push(Box::new(move |div, cx| {
            let callback = callback.clone();
            div.on_action(
                cx.listener(move |workspace, event, cx| (callback.clone())(workspace, event, cx)),
            )
        }));
        self
    }

    fn actions(&self, div: Div, cx: &mut ViewContext<Self>) -> Div {
        let mut div = div;
        for action in self.workspace_actions.iter() {
            div = (action)(div, cx)
        }
        div
    }
}

impl FocusableView for Workspace {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.active_pane.focus_handle(cx)
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let colors = theme.colors();
        let mut context = KeyContext::new_with_defaults();
        context.add("Workspace");

        self.actions(div(), cx)
            .key_context(context)
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(colors.text)
            .bg(colors.background)
            .children([TitleBar::new("titlebar")])
            .child(
                div()
                    .id("workspace")
                    .relative()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .border_t_1()
                    .border_b_1()
                    .border_color(colors.border)
                    .child(
                        div().child(div().flex().flex_col().flex_1().overflow_hidden().child(
                            h_flex().flex_1().child(self.center.render(
                                &self.active_pane,
                                None,
                                &self.app_state,
                                cx,
                            )),
                        )),
                    ),
            )
            .child(self.status_bar.clone())
    }
}
