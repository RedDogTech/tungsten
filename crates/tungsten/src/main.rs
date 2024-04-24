mod tungsten;

use std::sync::Arc;

use gpui::*;
use settings::SettingsStore;

use tungsten::{app_menus, build_window_options};
use workspace::AppState;

fn main() {
    env_logger::init();

    log::info!("========== starting tungsten ==========");

    let app = App::new();

    app.run(|cx: &mut AppContext| {
        let mut store = SettingsStore::default();

        cx.set_global(store);

        let app_state = Arc::new(AppState {
            build_window_options,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        let _ = workspace::new(app_state, cx);

        cx.set_menus(app_menus());

        cx.activate(true);
    });
}
