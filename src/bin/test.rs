use anyhow::{Error, Result};

use app_name::util::say_hello::hello;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    println!("{}", hello().await);

    Ok(())
}
