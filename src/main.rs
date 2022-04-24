
use std::time::Duration;

use tokio::{fs::File, io::AsyncReadExt, task, time};
use serde::{Serialize, Deserialize};
use anyhow::{Context, Result};

mod util;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub test: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config_env = std::env::var("RUST_CONFIG").context("read RUST_CONFIG env error")?;

    let mut fs = File::open(config_env).await.context("open config file error")?;
    let mut config_str = String::new();
    fs.read_to_string(&mut config_str).await.context("fs read_to_string error")?;

    log::debug!("config: {}", config_str);

    let config: Config = toml::from_str(config_str.as_str()).context("parse config error")?;
    log::info!("config.test: {:?}", config.test);
    

    let test_task = task::spawn(async move {
        let mut inte = time::interval(Duration::from_secs(2));
        loop {
            inte.tick().await;
            log::info!("test");
        }
    });

    // Block until ctrl-c is hit
    util::block_until_sigint().await;

    

    // Cancel all async services
    test_task.abort();


    log::info!("Finish shutdown.");

    Ok(())
}