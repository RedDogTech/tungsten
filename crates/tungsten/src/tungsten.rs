use std::sync::Arc;

use gpui::{
    actions, point, px, AppContext, Menu, MenuItem, PromptLevel, TitlebarOptions, WindowKind,
    WindowOptions,
};
use gpui::{FocusableView, VisualContext};
use theme::ActiveTheme;
use uuid::Uuid;
use workspace::{AppState, Workspace};

actions!(tungsten, [About, Quit]);

pub fn build_window_options(display_uuid: Option<Uuid>, cx: &mut AppContext) -> WindowOptions {
    let display = display_uuid.and_then(|uuid| {
        cx.displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    });

    WindowOptions {
        app_id: Some("tungsten".to_string()),
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        window_bounds: None,
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
        window_background: cx.theme().window_background_appearance(),
    }
}

pub fn app_menus() -> Vec<Menu<'static>> {
    vec![Menu {
        name: "tungsten",
        items: vec![
            MenuItem::action("About Tungsten…", About),
            MenuItem::action("Patches", patch_ui::Patch),
            MenuItem::action("Cues", cue_ui::Cue),
            MenuItem::action("Quit", Quit),
        ],
    }]
}

pub fn init(cx: &mut AppContext) {
    cx.on_action(quit);
}

pub fn initialize_workspace(app_state: Arc<AppState>, cx: &mut AppContext) {
    cx.observe_new_views(move |workspace: &mut Workspace, cx| {
        let workspace_handle = cx.view().clone();
        let center_pane = workspace.active_pane().clone();

        let dmx_activity = cx.new_view(|cx| dmx_output::items::DmxIndicator::new(workspace, cx));

        workspace.status_bar().update(cx, |status_bar, cx| {
            status_bar.add_left_item(dmx_activity, cx);
        });

        workspace.register_action(about);
        workspace.focus_handle(cx).focus(cx);
    })
    .detach();
}

fn about(_: &mut Workspace, _: &About, cx: &mut gpui::ViewContext<Workspace>) {
    let message = format!("This is a test");

    let prompt = cx.prompt(PromptLevel::Info, &message, None, &["OK"]);
    cx.foreground_executor()
        .spawn(async {
            prompt.await.ok();
        })
        .detach();
}

fn quit(_: &Quit, cx: &mut AppContext) {
    println!("Gracefully quitting the application . . .");
    cx.quit();
}
