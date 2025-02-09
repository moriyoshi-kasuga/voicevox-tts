use std::{collections::HashMap, io::Read, ops::Deref, sync::Arc};

use poise::serenity_prelude::{GuildId, UserId};
use tokio::sync::Mutex;

#[derive(Default, Clone)]
pub struct SpeakerDict(Arc<Mutex<HashMap<GuildId, HashMap<UserId, u32>>>>);

impl SpeakerDict {
    pub async fn set(&self, guild_id: GuildId, key: UserId, value: u32) {
        let mut map = self.0.lock().await;
        let map = map.entry(guild_id).or_default();
        map.insert(key, value);
    }

    pub async fn get(&self, guild_id: GuildId, key: UserId) -> Option<u32> {
        let map = self.0.lock().await;
        let map = map.get(&guild_id)?;
        map.get(&key).copied()
    }

    pub async fn remove(&self, guild_id: GuildId, key: &UserId) -> bool {
        let mut map = self.0.lock().await;
        let map = map.entry(guild_id).or_default();
        map.remove(key).is_some()
    }

    #[allow(clippy::unwrap_used)]
    pub async fn save(self) {
        let text = serde_json::to_string(self.0.lock().await.deref()).unwrap();
        std::fs::write(SPEAKER_DICT_PATH, text).unwrap();
    }
}

const SPEAKER_DICT_PATH: &str = "speaker_dict.json";

#[allow(clippy::unwrap_used)]
pub fn init_speaker_dict() -> SpeakerDict {
    if !std::fs::exists(SPEAKER_DICT_PATH).unwrap() {
        return SpeakerDict::default();
    }
    let mut file = std::fs::File::open(SPEAKER_DICT_PATH).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let map: HashMap<GuildId, HashMap<UserId, u32>> = serde_json::from_str(&text).unwrap();

    SpeakerDict(Arc::new(Mutex::new(map)))
}
