use gpui::{Font, Pixels};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::SettingsSources;

#[derive(Clone)]
pub struct ThemeSettings {
    pub ui_font_size: Pixels,
    pub ui_font: Font,
}

/// Settings for rendering text in UI and text buffers.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ThemeSettingsContent {
    /// The default font size for text in the UI.
    #[serde(default)]
    pub ui_font_size: Option<f32>,
    /// The name of a font to use for rendering in the UI.
    #[serde(default)]
    pub ui_font_family: Option<String>,
}

impl settings::Settings for ThemeSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = ThemeSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        cx: &mut gpui::AppContext,
    ) -> gpui::Result<Self>
    where
        Self: Sized,
    {
        let mut this = Self {
            ui_font_size: Pixels(14.),
            ui_font: Font {
                family: "Courier".into(),
                features: Default::default(),
                weight: Default::default(),
                style: Default::default(),
            },
        };
        Ok(this)
    }
}
