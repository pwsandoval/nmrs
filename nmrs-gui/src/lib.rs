pub mod file_lock;
pub mod objects;
pub mod style;
pub mod theme_config;
pub mod ui;

use clap::{ArgAction, Parser};
use gtk::prelude::*;
use gtk::Application;
use std::sync::OnceLock;

use crate::file_lock::acquire_app_lock;
use crate::style::load_css;
use crate::ui::build_ui;

#[derive(Parser, Debug)]
#[command(name = "nmrs")]
#[command(disable_version_flag = true)]
#[command(version)]
struct Args {
    #[arg(short = 'V', long = "version", action = ArgAction::SetTrue)]
    version: bool,

    #[arg(long = "wifi-only", action = ArgAction::SetTrue)]
    wifi_only: bool,

    #[arg(long = "bluetooth-only", action = ArgAction::SetTrue)]
    bluetooth_only: bool,
}

#[derive(Clone, Copy)]
pub struct UiPrefs {
    pub show_wifi: bool,
    pub show_bluetooth: bool,
}

static UI_PREFS: OnceLock<UiPrefs> = OnceLock::new();

pub fn ui_prefs() -> UiPrefs {
    *UI_PREFS.get().unwrap_or(&UiPrefs {
        show_wifi: true,
        show_bluetooth: true,
    })
}

pub fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Args { version: true, .. } = args {
        println!(
            "nmrs {}-beta ({})",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH")
        );
        return Ok(());
    }

    if args.wifi_only && args.bluetooth_only {
        anyhow::bail!("--wifi-only and --bluetooth-only are mutually exclusive");
    }

    let prefs = if args.wifi_only {
        UiPrefs {
            show_wifi: true,
            show_bluetooth: false,
        }
    } else if args.bluetooth_only {
        UiPrefs {
            show_wifi: false,
            show_bluetooth: true,
        }
    } else {
        UiPrefs {
            show_wifi: true,
            show_bluetooth: true,
        }
    };

    let _ = UI_PREFS.set(prefs);

    let app = Application::builder().application_id("org.nmrs.ui").build();

    let _lock = match acquire_app_lock() {
        Ok(lock) => lock,
        Err(e) => {
            eprintln!("Failed to start: {e}");
            std::process::exit(1);
        }
    };

    app.connect_activate(|app| {
        load_css();
        build_ui(app);
    });

    app.run();
    Ok(())
}
