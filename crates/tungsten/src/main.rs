use gpui::*;
use log::LevelFilter;
use settings::SettingsStore;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    env_logger::init();

    log::info!("========== starting tungsten ==========");

    let app = App::new();

    app.run(|cx: &mut AppContext| {
        let mut store = SettingsStore::default();

        cx.set_global(store);

        let _ = workspace::new(cx);

        cx.activate(true);
    });
}
