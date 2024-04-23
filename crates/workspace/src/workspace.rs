use gpui::{
    div, AnyView, AppContext, Global, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Task, ViewContext, VisualContext, WeakView, WindowContext, WindowOptions,
};
use std::sync::Weak;
use theme::ActiveTheme;
use ui::TitleBar;

pub struct AppState {}

struct GlobalAppState(Weak<AppState>);

impl Global for GlobalAppState {}

pub struct Workspace {
    weak_self: WeakView<Self>,
    titlebar_item: Option<AnyView>,
}

pub fn new(cx: &mut AppContext) {
    cx.spawn(|mut cx| async move {
        cx.open_window(WindowOptions::default(), {
            move |cx| cx.new_view(|cx| Workspace::new(cx))
        });
    })
    .detach();
}

impl Workspace {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
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
