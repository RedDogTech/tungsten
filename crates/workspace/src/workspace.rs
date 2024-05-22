use gpui::{
    actions, div, impl_actions, Action, AppContext, FocusHandle, FocusableView, Global,
    InteractiveElement, IntoElement, KeyContext, ParentElement, Render, Styled, Task, View,
    ViewContext, VisualContext, WeakView, WindowContext, WindowHandle, WindowOptions,
};
use item::ItemHandle;
use pane::Pane;
use pane_group::PaneGroup;
use serde::Deserialize;
use settings::Settings;
use std::sync::{atomic::AtomicUsize, Arc, Weak};
use theme::{ActiveTheme, ThemeSettings};
use ui::{
    h_flex, prelude::*, Button, ButtonCommon, ButtonStyle, Color, Div, FluentBuilder, LabelSize,
    TitleBar,
};
use uuid::Uuid;

pub mod item;
pub mod pane;
pub mod pane_group;
mod status_bar;
use status_bar::StatusBar;
pub use status_bar::StatusItemView;

impl_actions!(workspace, [ActivatePane]);
actions!(workspace, [NewWindow, CloseWindow]);

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
    app_state: Arc<AppState>,
    status_bar: View<StatusBar>,
}

pub fn init(app_state: Arc<AppState>, cx: &mut AppContext) {
    cx.on_action(Workspace::close_global);
}

impl Workspace {
    pub fn new(app_state: Arc<AppState>, cx: &mut ViewContext<Self>) -> Self {
        let weak_handle = cx.view().downgrade();

        cx.defer(|this, cx| {
            this.update_window_title(cx);
        });

        cx.on_focus_lost(|this, cx| {
            let focus_handle = this.focus_handle(cx);
            cx.focus(&focus_handle);
        })
        .detach();

        let history_timestamp = Arc::new(AtomicUsize::new(0));

        let center_pane = cx.new_view(|cx| Pane::new(weak_handle.clone(), history_timestamp, cx));

        cx.focus_view(&center_pane);

        let status_bar = cx.new_view(|cx| {
            let status_bar = StatusBar::new(&center_pane.clone(), cx);
            status_bar
        });

        Workspace {
            workspace_actions: Default::default(),
            weak_self: weak_handle.clone(),
            active_pane: center_pane.clone(),
            panes: vec![center_pane.clone()],
            center: PaneGroup::new(center_pane.clone()),
            app_state,
            status_bar,
        }
    }

    fn new_local(
        app_state: Arc<AppState>,
        requesting_window: Option<WindowHandle<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<anyhow::Result<WindowHandle<Workspace>>> {
        cx.spawn(|mut cx| async move {
            let window = if let Some(window) = requesting_window {
                cx.update_window(window.into(), |_, cx| {
                    cx.replace_root_view(|cx| Workspace::new(app_state.clone(), cx));
                })?;
                window
            } else {
                // Use the serialized workspace to construct the new window
                let options = cx.update(|cx| (app_state.build_window_options)(None, cx))?;

                cx.open_window(options, {
                    let app_state = app_state.clone();
                    move |cx| cx.new_view(|cx| Workspace::new(app_state, cx))
                })?
            };

            window.update(&mut cx, |_, cx| cx.activate_window())?;
            Ok(window)
        })
    }

    pub fn active_pane(&self) -> &View<Pane> {
        &self.active_pane
    }

    pub fn status_bar(&self) -> &View<StatusBar> {
        &self.status_bar
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
        let title = "empty project".to_string();
        cx.set_window_title(&title);
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

    fn title_bar(&self) -> impl IntoElement {
        TitleBar::new("titlebar")
            .when(cfg!(not(windows)), |this| {
                this.on_click(|event, cx| {
                    if event.up.click_count == 2 {
                        cx.zoom_window();
                    }
                })
            })
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("name_trigger", "Tungsten")
                            .style(ButtonStyle::Subtle)
                            .label_size(LabelSize::Small),
                    )
                    .child(
                        Button::new("project_trigger", "project name!")
                            .color(Color::Muted)
                            .style(ButtonStyle::Subtle)
                            .label_size(LabelSize::Small),
                    ),
            )
    }

    pub fn close_window(&mut self, _: &CloseWindow, cx: &mut ViewContext<Self>) {
        let window = cx.window_handle();
        cx.spawn(|_, mut cx| async move {
            window.update(&mut cx, |_, cx| {
                cx.remove_window();
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx)
    }

    pub fn close_global(_: &CloseWindow, cx: &mut AppContext) {
        cx.defer(|cx| {
            cx.windows().iter().find(|window| {
                window
                    .update(cx, |_, window| {
                        if window.is_window_active() {
                            //This can only get called when the window's project connection has been lost
                            //so we don't need to prompt the user for anything and instead just close the window
                            window.remove_window();
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            });
        });
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

        let (ui_font, ui_font_size) = {
            let theme_settings = ThemeSettings::get_global(cx);
            (
                theme_settings.ui_font.family.clone(),
                theme_settings.ui_font_size,
            )
        };

        cx.set_rem_size(ui_font_size);

        self.actions(div(), cx)
            .key_context(context)
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .font_family(ui_font)
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(colors.text)
            .bg(colors.background)
            .children([self.title_bar()])
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
                    .child(div().flex().flex_col().flex_1().overflow_hidden().child(
                        h_flex().flex_1().child(self.center.render(
                            &self.active_pane,
                            None,
                            &self.app_state,
                            cx,
                        )),
                    )),
            )
            .child(self.status_bar.clone())
    }
}

pub fn open_new(
    app_state: Arc<AppState>,
    cx: &mut AppContext,
    init: impl FnOnce(&mut Workspace, &mut ViewContext<Workspace>) + 'static + Send,
) -> Task<()> {
    let task = Workspace::new_local(app_state, None, cx);
    cx.spawn(|mut cx| async move {
        if let Some(workspace) = task.await.ok() {
            workspace
                .update(&mut cx, |workspace, cx| init(workspace, cx))
                .ok();
        }
    })
}
