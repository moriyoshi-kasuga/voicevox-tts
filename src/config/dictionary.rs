use std::{collections::HashMap, io::Read, ops::Deref, sync::Arc};

use tokio::sync::Mutex;

#[derive(Default, Clone)]
pub struct Dictionary(Arc<Mutex<HashMap<String, String>>>);

impl Dictionary {
    pub async fn set(&self, key: String, value: String) {
        self.0.lock().await.insert(key, value);
    }

    pub async fn replace(&self, text: &str) -> String {
        let map = self.0.lock().await;
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

const DICTIONARY_PATH: &str = "dictionary.toml";

#[allow(clippy::unwrap_used)]
pub fn init_dictionary() -> Dictionary {
    if !std::fs::exists(DICTIONARY_PATH).unwrap() {
        return Dictionary::default();
    }
    let mut file = std::fs::File::open(DICTIONARY_PATH).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    let map: HashMap<String, String> = toml::from_str(&text).unwrap();

    Dictionary(Arc::new(Mutex::new(map)))
}
