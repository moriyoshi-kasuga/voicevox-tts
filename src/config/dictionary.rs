use std::{
    collections::{hash_map::Entry, HashMap},
    io::Read,
    ops::Deref,
    sync::Arc,
};

use poise::serenity_prelude::{CreateEmbed, GuildId};
use tokio::sync::Mutex;

#[derive(Default, Clone)]
pub struct Dictionary(Arc<Mutex<HashMap<GuildId, HashMap<String, String>>>>);

impl Dictionary {
    pub async fn set(&self, guild_id: GuildId, key: String, value: String) -> bool {
        let mut map = self.0.lock().await;
        let map = map.entry(guild_id).or_default();
        match map.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(value);
                true
            }
            _ => false,
        }
    }

    pub async fn remove(&self, guild_id: GuildId, key: &str) -> bool {
        let mut map = self.0.lock().await;
        let map = map.entry(guild_id).or_default();
        map.remove(key).is_some()
    }

    pub async fn create_embed(&self, guild_id: GuildId) -> CreateEmbed {
        let embed = CreateEmbed::new();
        let mut map = self.0.lock().await;
        let map = map.entry(guild_id).or_default();

        embed.fields(map.iter().map(|(k, v)| (k, format!("`{v}`"), true)))
    }

    pub async fn replace(&self, guild_id: GuildId, text: &str) -> String {
        let mut map = self.0.lock().await;
        let map = map.entry(guild_id).or_default();
        let mut text = text.to_string();
        for (i, v) in map.iter() {
            text = text.replace(i, v);
        }
        text
    }

    #[allow(clippy::unwrap_used)]
    pub async fn save(self) {
        let text = toml::to_string(self.0.lock().await.deref()).unwrap();
        std::fs::write(DICTIONARY_PATH, text).unwrap();
    }
}

const DICTIONARY_PATH: &str = "dictionary.json";

#[allow(clippy::unwrap_used)]
pub fn init_dictionary() -> Dictionary {
    if !std::fs::exists(DICTIONARY_PATH).unwrap() {
        return Dictionary::default();
    }
    let mut file = std::fs::File::open(DICTIONARY_PATH).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let map: HashMap<GuildId, HashMap<String, String>> = serde_json::from_str(&text).unwrap();

    Dictionary(Arc::new(Mutex::new(map)))
}
