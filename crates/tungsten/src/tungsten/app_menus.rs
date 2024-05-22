use gpui::{Menu, MenuItem};

use crate::tungsten::{About, Minimize, Quit, Zoom};

pub fn app_menus() -> Vec<Menu<'static>> {
    vec![
        Menu {
            name: "tungsten",
            items: vec![
                MenuItem::action("About Tungsten…", About),
                MenuItem::action("Patches", patch_ui::Patch),
                MenuItem::action("Cues", cue_ui::Cue),
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            name: "File",
            items: vec![
                MenuItem::action("New Window", workspace::NewWindow),
                MenuItem::action("Close Window", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "Window",
            items: vec![
                MenuItem::action("Minimize", Minimize),
                MenuItem::action("Zoom", Zoom),
                MenuItem::separator(),
            ],
        },
    ]
}
