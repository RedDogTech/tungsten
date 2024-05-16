mod tungsten;

use assets::Assets;
use gpui::*;
use parking_lot::Mutex;
use std::sync::Arc;
use tungsten::{app_menus, build_window_options};
use workspace::AppState;

fn main() {
    env_logger::init();

    log::info!("========== starting tungsten ==========");

    let app = App::new().with_assets(Assets);

    app.run(|cx: &mut AppContext| {
        load_embedded_fonts(cx);
        settings::init(cx);

        theme::init(cx);

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

            println!("adding {:?}", font_path);

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
