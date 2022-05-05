
extern crate template;
use anyhow::{Result, Ok};

use template::test_module::test_module_fn;

#[tokio::main]
async fn main() -> Result<()> {

    test_module_fn();
    
    
    Ok(())
}
