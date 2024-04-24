use gpui::{
    div, AnyView, AppContext, Global, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Task, ViewContext, VisualContext, WeakView, WindowContext, WindowOptions,
};
use std::sync::{Arc, Weak};
use theme::ActiveTheme;
use ui::TitleBar;
use uuid::Uuid;

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
    weak_self: WeakView<Self>,
    titlebar_item: Option<AnyView>,
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

        Workspace {
            weak_self: weak_handle.clone(),
            titlebar_item: None,
        }
    }

    fn update_window_title(&mut self, cx: &mut WindowContext) {
        let mut title = String::new();
        title = "empty project".to_string();
        cx.set_window_title(&title);
    }

    fn titlebar_item(&mut self) -> impl IntoElement {
        TitleBar::new("collab-titlebar")
    }
}

impl Render for Workspace {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let colors = theme.colors();

        div()
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
                    .border_t()
                    .border_b()
                    .border_color(colors.border)
                    .child("Hello world"),
            )
    }
}
