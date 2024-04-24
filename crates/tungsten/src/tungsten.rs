use gpui::{point, px, AppContext, Menu, MenuItem, TitlebarOptions, WindowKind, WindowOptions};
use theme::ActiveTheme;
use uuid::Uuid;

pub fn build_window_options(display_uuid: Option<Uuid>, cx: &mut AppContext) -> WindowOptions {
    let display = display_uuid.and_then(|uuid| {
        cx.displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    });

    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        bounds: None,
        focus: true,
        show: true,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
        fullscreen: false,
        window_background: cx.theme().window_background_appearance(),
    }
}

pub fn app_menus() -> Vec<Menu<'static>> {
    vec![Menu {
        name: "Tungsten",
        items: vec![],
    }]
}
