use std::sync::Arc;

use config::save_on_sigint;
use tokio::sync::Mutex;
use ui::ui;

use crate::config::Config;

mod api;
mod config;
mod ui;

#[tokio::main]
async fn main() {
    let conf = Arc::new(Mutex::new(config::load().unwrap_or_else(|x| {
        println!("Cannot use config file: {x}\nUsing empty defaults.");
        Config::default()
    })));
    let conf_ref = conf.clone();
    tokio::spawn(async move {
        if let Err(why) = save_on_sigint(conf_ref).await {
            println!("Cannot save to config file: {why}")
        }
    });
    ui(conf.clone()).await
}
