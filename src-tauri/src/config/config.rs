use anyhow::Result;
use once_cell::sync::OnceCell;

use super::{motrix::IMotrix, Draft, IAria2Temp};

pub struct Config {
    motrix_config: Draft<IMotrix>,
    aria2_config: Draft<IAria2Temp>,
}

impl Config {
    pub fn global() -> &'static Config {
        static CONFIG: OnceCell<Config> = OnceCell::new();

        CONFIG.get_or_init(|| Config {
            motrix_config: Draft::from(IMotrix::new()),
            aria2_config: Draft::from(IAria2Temp::new()),
        })
    }

    pub async fn init_config() -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn motrix() -> Draft<IMotrix> {
        Self::global().motrix_config.clone()
    }

    pub fn aria2() -> Draft<IAria2Temp> {
        Self::global().aria2_config.clone()
    }
}

// #[derive(Debug)]
// pub enum ConfigType {
//     Run,
//     Check,
// }
