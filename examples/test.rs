
extern crate template;
use anyhow::{Result, Ok};

use template::util::hello;

#[tokio::main]
async fn main() -> Result<()> {

    println!("{}", hello().await);
    
    
    Ok(())
}
