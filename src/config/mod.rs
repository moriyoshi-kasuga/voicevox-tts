use core::panic;
use std::io::Read;

use messages::VoiceConfig;
use serde::de::DeserializeOwned;
use serde_inline_default::serde_inline_default;

pub mod messages;

const CONFIG_PATH: &str = "config.toml";

#[test]
fn test_default_config() {
    BotConfig::gen_default_config();
}

#[serde_inline_default]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BotConfig {
    #[serde_inline_default(20)]
    pub max_voice_cache: u64,
    #[serde_inline_default(VoiceConfig::gen_default_config())]
    pub voice: VoiceConfig,
}

trait DefaultConfig: DeserializeOwned {
    fn gen_default_config() -> Self {
        #[allow(clippy::unwrap_used)]
        toml::from_str("").unwrap()
    }
}

impl DefaultConfig for BotConfig {}

#[allow(clippy::unwrap_used, clippy::panic)]
pub fn init_config() -> BotConfig {
    tracing::info!("loading config");

    if !std::fs::exists(CONFIG_PATH).unwrap() {
        let text = toml::to_string(&BotConfig::gen_default_config()).unwrap();
        std::fs::write(CONFIG_PATH, text).unwrap();

        panic!("config.toml not found, generated default config. please set discord token");
    };
    let mut file = std::fs::File::open(CONFIG_PATH).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let config = toml::from_str(&text).unwrap();

    tracing::info!("loaded config");

    config
}
