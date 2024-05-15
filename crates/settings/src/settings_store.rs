use anyhow::{anyhow, Context, Result};
use gpui::{AppContext, BorrowAppContext, Global};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize as _, Serialize};
use std::{
    any::{type_name, Any, TypeId},
    collections::{hash_map, HashMap},
    path::Path,
    sync::Arc,
};

pub trait Settings: 'static + Send + Sync {
    /// The name of a key within the JSON file from which this setting should
    /// be deserialized. If this is `None`, then the setting will be deserialized
    /// from the root object.
    const KEY: Option<&'static str>;

    type FileContent: Clone + Default + Serialize + DeserializeOwned + JsonSchema;

    fn load(sources: SettingsSources<Self::FileContent>, cx: &mut AppContext) -> Result<Self>
    where
        Self: Sized;

    fn register(cx: &mut AppContext)
    where
        Self: Sized,
    {
        cx.update_global(|store: &mut SettingsStore, cx| {
            store.register_setting::<Self>(cx);
        });
    }

    fn get_global(cx: &AppContext) -> &Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get()
    }
}

pub struct SettingsStore {
    setting_values: HashMap<TypeId, Box<dyn AnySettingValue>>,
    raw_default_settings: serde_json::Value,
    raw_user_settings: serde_json::Value,
}

impl Global for SettingsStore {}

impl Default for SettingsStore {
    fn default() -> Self {
        SettingsStore {
            setting_values: Default::default(),
            raw_default_settings: serde_json::json!({}),
            raw_user_settings: serde_json::json!({}),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SettingsSources<'a, T> {
    /// The default Zed settings.
    pub default: &'a T,
    pub user: Option<&'a T>,
}

impl<'a, T: Serialize> SettingsSources<'a, T> {
    /// Returns an iterator over the default settings as well as all settings customizations.
    pub fn defaults_and_customizations(&self) -> impl Iterator<Item = &T> {
        [self.default].into_iter().chain(self.customizations())
    }

    /// Returns an iterator over all of the settings customizations.
    pub fn customizations(&self) -> impl Iterator<Item = &T> {
        self.user.into_iter()
    }

    /// Returns the settings after performing a JSON merge of the provided customizations.
    ///
    /// Customizations later in the iterator win out over the earlier ones.
    pub fn json_merge_with<O: DeserializeOwned>(
        customizations: impl Iterator<Item = &'a T>,
    ) -> Result<O> {
        let mut merged = serde_json::Value::Null;
        for value in customizations {
            merge_non_null_json_value_into(serde_json::to_value(value).unwrap(), &mut merged);
        }
        Ok(serde_json::from_value(merged)?)
    }

    /// Returns the settings after performing a JSON merge of the customizations into the
    /// default settings.
    ///
    /// More-specific customizations win out over the less-specific ones.
    pub fn json_merge<O: DeserializeOwned>(&'a self) -> Result<O> {
        Self::json_merge_with(self.defaults_and_customizations())
    }
}

#[derive(Debug)]
struct SettingValue<T> {
    global_value: Option<T>,
    local_values: Vec<(usize, Arc<Path>, T)>,
}

struct DeserializedSetting(Box<dyn Any>);

trait AnySettingValue: 'static + Send + Sync {
    fn deserialize_setting(&self, json: &serde_json::Value) -> Result<DeserializedSetting>;
    fn setting_type_name(&self) -> &'static str;
    fn value_for_path(&self) -> &dyn Any;
    fn key(&self) -> Option<&'static str>;
    fn set_global_value(&mut self, value: Box<dyn Any>);
    fn load_setting(
        &self,
        sources: SettingsSources<DeserializedSetting>,
        cx: &mut AppContext,
    ) -> Result<Box<dyn Any>>;
}

impl<T: Settings> AnySettingValue for SettingValue<T> {
    fn key(&self) -> Option<&'static str> {
        T::KEY
    }

    fn setting_type_name(&self) -> &'static str {
        type_name::<T>()
    }

    fn value_for_path(&self) -> &dyn Any {
        self.global_value
            .as_ref()
            .unwrap_or_else(|| panic!("no default value for setting {}", self.setting_type_name()))
    }

    fn deserialize_setting(&self, mut json: &serde_json::Value) -> Result<DeserializedSetting> {
        if let Some(key) = T::KEY {
            if let Some(value) = json.get(key) {
                json = value;
            } else {
                let value = T::FileContent::default();
                return Ok(DeserializedSetting(Box::new(value)));
            }
        }
        let value = T::FileContent::deserialize(json)?;
        Ok(DeserializedSetting(Box::new(value)))
    }

    fn set_global_value(&mut self, value: Box<dyn Any>) {
        self.global_value = Some(*value.downcast().unwrap());
    }

    fn load_setting(
        &self,
        values: SettingsSources<DeserializedSetting>,
        cx: &mut AppContext,
    ) -> Result<Box<dyn Any>> {
        Ok(Box::new(T::load(
            SettingsSources {
                default: values.default.0.downcast_ref::<T::FileContent>().unwrap(),
                user: values
                    .user
                    .map(|value| value.0.downcast_ref::<T::FileContent>().unwrap()),
            },
            cx,
        )?))
    }
}

impl SettingsStore {
    /// Add a new type of setting to the store.
    pub fn register_setting<T: Settings>(&mut self, cx: &mut AppContext) {
        let setting_type_id = TypeId::of::<T>();
        let entry = self.setting_values.entry(setting_type_id);

        if matches!(entry, hash_map::Entry::Occupied(_)) {
            return;
        }

        let setting_value = entry.or_insert(Box::new(SettingValue::<T> {
            global_value: None,
            local_values: Vec::new(),
        }));

        if let Ok(default_settings) = setting_value.deserialize_setting(&self.raw_default_settings)
        {
            if let Some(setting) = setting_value
                .load_setting(
                    SettingsSources {
                        default: &default_settings,
                        user: None,
                    },
                    cx,
                )
                .context("A default setting must be added to the `default.json` file")
                .ok()
            {
                setting_value.set_global_value(setting);
            }
        }
    }

    pub fn set_default_settings(
        &mut self,
        default_settings_content: &str,
        cx: &mut AppContext,
    ) -> Result<()> {
        let settings: serde_json::Value = parse_json_with_comments(default_settings_content)?;

        if settings.is_object() {
            self.raw_default_settings = settings;
            self.recompute_values(cx)?;
            Ok(())
        } else {
            Err(anyhow!("settings must be an object"))
        }
    }

    /// Sets the user settings via a JSON string.
    pub fn set_user_settings(
        &mut self,
        user_settings_content: &str,
        cx: &mut AppContext,
    ) -> Result<()> {
        let settings: serde_json::Value = if user_settings_content.is_empty() {
            parse_json_with_comments("{}")?
        } else {
            parse_json_with_comments(user_settings_content)?
        };
        if settings.is_object() {
            self.raw_user_settings = settings;
            self.recompute_values(cx)?;
            Ok(())
        } else {
            Err(anyhow!("settings must be an object"))
        }
    }

    /// Get the value of a setting.
    ///
    /// Panics if the given setting type has not been registered, or if there is no
    /// value for this setting.
    pub fn get<T: Settings>(&self) -> &T {
        self.setting_values
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()))
            .value_for_path()
            .downcast_ref::<T>()
            .expect("no default value for setting type")
    }

    fn recompute_values(&self, cx: &mut AppContext) -> Result<()> {
        Ok(())
    }
}

pub fn parse_json_with_comments<T: DeserializeOwned>(content: &str) -> Result<T> {
    Ok(serde_json_lenient::from_str(content)?)
}

fn merge_non_null_json_value_into(source: serde_json::Value, target: &mut serde_json::Value) {
    use serde_json::Value;
    if let Value::Object(source_object) = source {
        let target_object = if let Value::Object(target) = target {
            target
        } else {
            *target = Value::Object(Default::default());
            target.as_object_mut().unwrap()
        };
        for (key, value) in source_object {
            if let Some(target) = target_object.get_mut(&key) {
                merge_non_null_json_value_into(value, target);
            } else if !value.is_null() {
                target_object.insert(key.clone(), value);
            }
        }
    } else if !source.is_null() {
        *target = source
    }
}
