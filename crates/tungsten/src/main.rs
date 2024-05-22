mod tungsten;

use assets::Assets;
use gpui::*;
use parking_lot::Mutex;
use std::sync::Arc;
use tungsten::{app_menus, build_window_options};
use workspace::{open_new, AppState};

fn main() {
    env_logger::init();

    log::info!("========== starting tungsten ==========");

    let app = App::new().with_assets(Assets);

    app.run(|cx: &mut AppContext| {
        let app_state = Arc::new(AppState {
            build_window_options,
        });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        settings::init(cx);
        tungsten::init(cx);
        load_embedded_fonts(cx);
        theme::init(cx);
        workspace::init(app_state.clone(), cx);
        dmx_output::init(cx);
        patch_ui::init(cx);
        cue_ui::init(cx);

        cx.set_menus(app_menus());
        tungsten::initialize_workspace(app_state.clone(), cx);
        cx.activate(true);

        open_new(app_state, cx, |_, _| {}).detach();
    });
}

fn load_embedded_fonts(cx: &AppContext) {
    let asset_source = cx.asset_source();
    let font_paths = asset_source.list("fonts").unwrap();
    let embedded_fonts = Mutex::new(Vec::new());
    let executor = cx.background_executor();

    executor.block(executor.scoped(|scope| {
        for font_path in &font_paths {
            if !font_path.ends_with(".ttf") {
                continue;
            }

            scope.spawn(async {
                let font_bytes = asset_source.load(font_path).unwrap();
                embedded_fonts.lock().push(font_bytes);
            });
        }
    }));

    cx.text_system()
        .add_fonts(embedded_fonts.into_inner())
        .unwrap();
}
