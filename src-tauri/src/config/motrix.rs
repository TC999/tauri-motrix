use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    logging,
    service::i18n,
    utils::{dirs, help, logging::Type},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IMotrix {
    /// aria2c run name for sidecar
    pub aria2_engine: Option<String>,

    pub app_hide_window: Option<bool>,

    // i18n
    pub language: Option<String>,

    /// app log level
    /// silent | error | warn | info | debug | trace
    pub app_log_level: Option<String>,

    /// `light` or `dark` or `system`
    pub theme_mode: Option<String>,
    /// 0 -> no clear
    /// 1 -> 7 day
    /// 2 -> 30 day
    /// 3 -> 90 day
    pub auto_log_clean: Option<i32>,

    pub enable_auto_launch: Option<bool>,
}

impl IMotrix {
    pub fn new() -> Self {
        let template = Self::template();

        match dirs::motrix_path().and_then(|path| help::read_yaml::<IMotrix>(&path)) {
            Ok(mut config) => {
                logging!(info, Type::Core, true, "Loaded config: {:?}", config);
                let template = serde_yaml::to_value(template).unwrap_or_default();
                let mut config_value = serde_yaml::to_value(&config).unwrap_or_default();

                if let Some(template_map) = template.as_mapping() {
                    if let Some(config_map) = config_value.as_mapping_mut() {
                        for (key, value) in template_map {
                            config_map
                                .entry(key.clone())
                                .or_insert_with(|| value.clone());
                        }
                    }
                }

                config = serde_yaml::from_value(config_value).unwrap_or(config);
                config
            }
            Err(err) => {
                logging!(error, Type::Core, true, "{err}");
                template
            }
        }
    }

    pub fn template() -> Self {
        IMotrix {
            aria2_engine: Some("aria2c".into()),
            app_hide_window: Some(false),
            language: i18n::get_system_language().into(),
            theme_mode: Some("system".into()),
            app_log_level: Some("info".into()),
            enable_auto_launch: Some(false),
            ..Self::default()
        }
    }

    /// Save IMotrix App Config
    pub fn save_file(&self) -> Result<()> {
        help::save_yaml(&dirs::motrix_path()?, &self, Some("# tauri-motrix Config"))
    }

    /// patch motrix config
    /// only save to file
    ///
    pub fn patch_config(&mut self, patch: IMotrix) {
        macro_rules! patch {
            ($key: ident) => {
                if patch.$key.is_some() {
                    self.$key = patch.$key;
                }
            };
        }

        patch!(theme_mode);
        patch!(app_log_level);
        patch!(aria2_engine);
        patch!(app_hide_window);
        patch!(language);
        patch!(enable_auto_launch);
    }
}
