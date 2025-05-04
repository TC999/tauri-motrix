use std::collections::HashMap;

use anyhow::Result;
use serde_json::json;

use crate::{
    config::{Config, IMotrix},
    core::sys_opt,
    service,
};

// Define update flags as bitflags for better performance
#[derive(Clone, Copy)]
enum UpdateFlags {
    None = 0,
    // RestartCore = 1 << 0,
    // Aria2Config = 1 << 1,
    // MotrixConfig = 1 << 2,
    Launch = 1 << 3,
    HideWindow = 1 << 4,
    TrayMenu = 1 << 5,
}

/// expose outside for motrix config
pub async fn patch_motrix(data: IMotrix) -> Result<()> {
    Config::motrix().draft().patch_config(data.clone());

    let language = data.language;
    let auto_launch = data.enable_auto_launch;
    let app_hide_window = data.app_hide_window;

    let res: Result<()> = {
        let mut flag_signal: i32 = UpdateFlags::None as i32;

        if language.is_some() {
            flag_signal |= UpdateFlags::TrayMenu as i32;
        }

        if auto_launch.is_some() {
            flag_signal |= UpdateFlags::Launch as i32;
        }

        if app_hide_window.is_some() {
            flag_signal |= UpdateFlags::HideWindow as i32;
        }

        // Process updates based on flags
        if (flag_signal & (UpdateFlags::Launch as i32)) != 0 {
            sys_opt::SysOpt::global().update_launch()?;
        }

        if (flag_signal & (UpdateFlags::HideWindow as i32)) != 0 {
            // TODO
        }

        if (flag_signal & (UpdateFlags::TrayMenu as i32)) != 0 {
            service::tray::update_tray_menu()?;
        }

        Ok(())
    };

    match res {
        Ok(()) => {
            Config::motrix().apply();
            Config::motrix().data().save_file()?;
            Ok(())
        }
        Err(err) => {
            Config::motrix().discard();
            Err(err)
        }
    }
}

pub async fn patch_aria2(data: HashMap<String, String>) -> Result<()> {
    Config::aria2().draft().patch_config(data.clone());

    // TODO: check conf

    let res = {
        service::aria2c::change_global_option(&[json!(data)]).await?;
        <Result<()>>::Ok(())
    };

    match res {
        Ok(()) => {
            Config::aria2().apply();
            let _ = Config::aria2().data().save_file();

            Ok(())
        }
        Err(err) => {
            Config::aria2().discard();
            Err(err)
        }
    }
}
