mod tungsten;

use assets::Assets;
use gpui::*;
use std::sync::Arc;
use tungsten::{app_menus, build_window_options};
use workspace::AppState;

fn main() {
    env_logger::init();

    log::info!("========== starting tungsten ==========");

    let app = App::new().with_assets(Assets);

    app.run(|cx: &mut AppContext| {
        settings::init(cx);

        let app_state = Arc::new(AppState {
            build_window_options,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        let _ = workspace::new(app_state.clone(), cx);

        dmx_output::init(cx);

        tungsten::init(cx);
        tungsten::initialize_workspace(app_state.clone(), cx);
        patch_ui::init(cx);
        cue_ui::init(cx);

        cx.set_menus(app_menus());
        cx.activate(true);
    });
}
