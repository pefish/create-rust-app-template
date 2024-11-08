use std::time::Duration;

use anyhow::{Context, Error, Result};
use tokio::time;

mod util;
use util::hello;

mod util_test;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();

    let test = std::env::var("TEST").expect("TEST env must be set.");

    log::info!("test: {}", test);

    log::info!("{}", hello().await);

    block_until_sigint::block(async move {
        let mut inte = time::interval(Duration::from_secs(2));
        let mut tmp = 0;
        loop {
            inte.tick().await;
            log::info!("test");
            tmp += 1;
            if tmp == 5 {
                break;
            }
        }
    })
    .await
    .context("block_until_sigint error")?;

    log::info!("Finish shutdown.");

    Ok(())
}
