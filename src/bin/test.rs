extern crate template;

use anyhow::{Context, Result, Error};

use template::util::hello;



#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    
    println!("{}", hello().await);

    Ok(())
}
