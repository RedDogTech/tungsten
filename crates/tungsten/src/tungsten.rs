use std::sync::Arc;

use gpui::{
    actions, point, px, AppContext, PromptLevel, TitlebarOptions, WindowKind, WindowOptions,
};
use gpui::{FocusableView, VisualContext};
use theme::ActiveTheme;
use uuid::Uuid;
use workspace::{open_new, AppState, NewWindow, Workspace};

mod app_menus;

pub use app_menus::*;

actions!(tungsten, [About, Minimize, Quit, Zoom, ToggleFullScreen]);

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
        focus: false,
        show: false,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
        window_background: cx.theme().window_background_appearance(),
    }
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

        let handle = cx.view().downgrade();
        cx.on_window_should_close(move |cx| {
            handle
                .update(cx, |workspace, cx| {
                    // We'll handle closing asynchronously
                    workspace.close_window(&Default::default(), cx);
                    false
                })
                .unwrap_or(true)
        });

        cx.spawn(|workspace_handle, mut cx| async move {
            workspace_handle.update(&mut cx, |_, cx| {
                cx.focus_self();
            })
        })
        .detach();

        workspace
            .register_action(about)
            .register_action(|_, _: &Minimize, cx| {
                cx.minimize_window();
            })
            .register_action(|_, _: &Zoom, cx| {
                cx.zoom_window();
            })
            .register_action(|_, _: &ToggleFullScreen, cx| {
                cx.toggle_fullscreen();
            })
            .register_action({
                let app_state = Arc::downgrade(&app_state);
                move |_, _: &NewWindow, cx| {
                    if let Some(app_state) = app_state.upgrade() {
                        open_new(app_state, cx, |_, _| {}).detach();
                    }
                }
            });

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
    let should_confirm = true;
    cx.spawn(|mut cx| async move {
        let workspace_windows = cx.update(|cx| {
            cx.windows()
                .into_iter()
                .filter_map(|window| window.downcast::<Workspace>())
                .collect::<Vec<_>>()
        })?;

        if let (true, Some(workspace)) = (should_confirm, workspace_windows.first().copied()) {
            let answer = workspace
                .update(&mut cx, |_, cx| {
                    cx.prompt(
                        PromptLevel::Info,
                        "Are you sure you want to quit?",
                        None,
                        &["Quit", "Cancel"],
                    )
                })
                .ok();

            if let Some(answer) = answer {
                let answer = answer.await.ok();
                if answer != Some(0) {
                    return Ok(());
                }
            }
        }
        cx.update(|cx| cx.quit())?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
