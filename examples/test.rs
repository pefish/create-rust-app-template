
extern crate template;
use anyhow::{Result, Ok};

use template::module::hello;

#[tokio::main]
async fn main() -> Result<()> {

    println!("{}", hello().await);
    
    
    Ok(())
}
