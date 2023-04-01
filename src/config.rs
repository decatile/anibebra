use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
    sync::Arc,
};

use make_fields_public::public;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Default)]
#[public]
pub struct Config {
    watching: Vec<View>,
    done: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
#[public]
pub struct View {
    title_id: i32,
    episode: i32,
    timecode: i32,
    filepath: PathBuf,
}

pub fn load() -> anyhow::Result<Config> {
    let file = File::open(get_config_path())?;
    Ok(serde_json::from_reader::<_, Config>(file)?)
}

pub async fn save_on_sigint(config: Arc<Mutex<Config>>) -> anyhow::Result<()> {
    tokio::signal::ctrl_c().await?;
    let path = get_config_path();
    fs::create_dir_all(path.parent().unwrap())?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)?;
    let conf = config.lock().await;
    Ok(serde_yaml::to_writer(file, &*conf)?)
}

#[cfg(target_os = "windows")]
fn get_config_path() -> PathBuf {
    "%AppData%/anibebra/config.yaml".into()
}

#[cfg(target_os = "linux")]
fn get_config_path() -> PathBuf {
    "~/.config/anibebra/config.yaml".into()
}
