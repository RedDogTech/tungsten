mod settings_store;

use gpui::AppContext;
use rust_embed::RustEmbed;
use std::borrow::Cow;

pub use settings_store::{Settings, SettingsSources, SettingsStore};

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "settings/*"]
#[include = "keymaps/*"]
#[exclude = "*.DS_Store"]
pub struct SettingsAssets;

pub fn init(cx: &mut AppContext) {
    let mut settings = SettingsStore::default();
    settings
        .set_default_settings(&default_settings(), cx)
        .unwrap();
    cx.set_global(settings);
}

pub fn default_settings() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/default.json")
}

pub fn user_settings() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/user.json")
}

pub fn asset_str<A: rust_embed::RustEmbed>(path: &str) -> Cow<'static, str> {
    match A::get(path).unwrap().data {
        Cow::Borrowed(bytes) => Cow::Borrowed(std::str::from_utf8(bytes).unwrap()),
        Cow::Owned(bytes) => Cow::Owned(String::from_utf8(bytes).unwrap()),
    }
}
