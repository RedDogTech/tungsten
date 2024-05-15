use anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::SettingsSources;

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub(crate) struct DmxOuputSettings {
    pub(crate) enable_artnet: bool,
    pub(crate) enable_sacn: bool,
}

/// Task-related settings.
#[derive(Serialize, Deserialize, PartialEq, Default, Clone, JsonSchema)]
pub(crate) struct DmxOuputSettingsContent {
    enable_artnet: Option<bool>,
    enable_sacn: Option<bool>,
}

impl settings::Settings for DmxOuputSettings {
    const KEY: Option<&'static str> = Some("dmx_output");

    type FileContent = DmxOuputSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }
}
