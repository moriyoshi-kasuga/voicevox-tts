use std::{collections::HashMap, sync::Arc};

use songbird::{
    id::{ChannelId, GuildId},
    typemap::TypeMapKey,
};
use tokio::sync::Mutex;

#[derive(Default, Clone)]
pub struct TtsChannel(Arc<Mutex<HashMap<GuildId, ChannelId>>>);

impl TtsChannel {
    pub async fn set(&self, guild_id: GuildId, channel_id: ChannelId) {
        self.0.lock().await.insert(guild_id, channel_id);
    }

    pub async fn remove(&self, guild_id: GuildId) {
        self.0.lock().await.remove(&guild_id);
    }

    pub async fn get(&self, guild_id: GuildId) -> Option<ChannelId> {
        self.0.lock().await.get(&guild_id).copied()
    }

    pub async fn has_eq(&self, guild_id: GuildId, channel_id: ChannelId) -> bool {
        self.0
            .lock()
            .await
            .get(&guild_id)
            .is_some_and(|v| *v == channel_id)
    }
}

pub struct TtsChannelKey;

impl TypeMapKey for TtsChannelKey {
    type Value = TtsChannel;
}

pub type VoiceCache = moka::future::Cache<(Box<str>, u32), Vec<u8>>;

pub struct VoiceCacheKey;

impl TypeMapKey for VoiceCacheKey {
    type Value = VoiceCache;
}
