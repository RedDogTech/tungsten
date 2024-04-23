use gpui::Global;
use std::{path::Path, sync::Arc};

pub trait Settings: 'static + Send + Sync {
    /// The name of a key within the JSON file from which this setting should
    /// be deserialized. If this is `None`, then the setting will be deserialized
    /// from the root object.
    const KEY: Option<&'static str>;
}

#[derive(Debug)]
struct SettingValue<T> {
    global_value: Option<T>,
    local_values: Vec<(usize, Arc<Path>, T)>,
}

trait AnySettingValue: 'static + Send + Sync {
    fn key(&self) -> Option<&'static str>;
}

pub struct SettingsStore {}

impl Global for SettingsStore {}

impl Default for SettingsStore {
    fn default() -> Self {
        SettingsStore {}
    }
}

impl SettingsStore {}
