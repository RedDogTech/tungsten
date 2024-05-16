mod dmx_output_settings;
pub mod items;

use dmx_output_settings::DmxOuputSettings;
use gpui::AppContext;
use settings::Settings;

pub fn init(cx: &mut AppContext) {
    DmxOuputSettings::register(cx);
}
