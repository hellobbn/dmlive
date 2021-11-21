mod config;
mod utils;
mod danmaku;
mod dmlive;
mod ipcmanager;
mod ffmpeg;
mod streamer;
mod streamfinder;
mod mpv;

use std::sync::Arc;

use clap::Arg;
use log::*;
use tokio::runtime::Builder;

fn main() {
    let ma = clap::App::new("dmlive")
        .arg(
            Arg::with_name("room-url")
                .short("u")
                .long("room-url")
                .value_name("STRING")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("log-level")
                .long("log-level")
                .required(false)
                .takes_value(true),
        )
        .version(clap::crate_version!())
        .get_matches();
    let log_level = match ma.value_of("log-level").unwrap_or("3").parse().unwrap_or(3) {
        1 => LevelFilter::Debug,
        2 => LevelFilter::Info,
        3 => LevelFilter::Warn,
        4 => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    env_logger::Builder::new().filter(None, log_level).init();

    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let proj_dirs = directories::ProjectDirs::from("com", "THMonster", "dmlive").unwrap();
            let d = proj_dirs.config_dir();
            let _ = tokio::fs::create_dir_all(&d).await;
            let config_path = d.join("config.toml");
            let _ = tokio::fs::File::create(&config_path).await;
            let cm = Arc::new(crate::config::ConfigManager::new(
                config_path,
                ma.value_of("room-url").unwrap(),
            ));
            let dml = dmlive::DMLive::new(cm).await;
            let dml = Arc::new(dml);
            dml.run().await;
        })
}