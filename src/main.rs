
use tokio::{fs::File, io::AsyncReadExt};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub test: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config_env = std::env::var("RUST_CONFIG")?;

    let mut fs = File::open(config_env).await?;
    let mut config_str = String::new();
    fs.read_to_string(&mut config_str).await?;

    log::debug!("config: {}", config_str);

    let config: Config = toml::from_str(config_str.as_str())?;
    log::info!("config.test: {:?}", config.test);
    
    Ok(())
}