use std::sync::Arc;

use poise::serenity_prelude::GuildId;
use serde_inline_default::serde_inline_default;

use macros::gen_message;

use crate::{util::vvc::gen_tts, AnyResult};

use super::DefaultConfig;

pub type Message = Vec<MessageFormat>;

#[serde_inline_default]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct VoiceConfig {
    #[serde_inline_default(2)]
    pub default_speaker_id: u32,
    #[serde_inline_default(60)]
    pub max_message_length: usize,
    #[serde_inline_default("以下略".to_string())]
    pub overed_message: String,

    #[serde_inline_default(gen_message!({0},"さんが参加しました"))]
    pub join: ConstMessage<1>,
    #[serde_inline_default(gen_message!({0},"さんが退出しました"))]
    pub leave: ConstMessage<1>,
}

impl DefaultConfig for VoiceConfig {}

#[derive(Debug)]
pub enum MessageFormat {
    Text(String),
    Arg(usize),
}

/// N is argument length
#[derive(Debug)]
pub struct ConstMessage<const N: usize>(Vec<MessageFormat>);

impl<const N: usize> ConstMessage<N> {
    pub fn format(&self, args: &[&str; N]) -> String {
        let mut text = String::new();

        for i in &self.0 {
            match i {
                MessageFormat::Text(s) => text.push_str(s),
                MessageFormat::Arg(n) => text.push_str(args[*n]),
            }
        }

        text
    }

    pub async fn process(
        &self,
        bot_data: Arc<crate::BotData>,
        guild_id: GuildId,
        speaker_id: u32,
        args: &[&str; N],
    ) -> AnyResult<Vec<u8>> {
        let mut all_voice = Vec::<u8>::new();

        for i in &self.0 {
            let text = match i {
                MessageFormat::Text(s) => s,
                MessageFormat::Arg(n) => args[*n],
            };

            let voice = gen_tts(text, bot_data.clone(), guild_id, speaker_id).await?;

            all_voice.extend(voice);
        }

        Ok(all_voice)
    }
}

impl<'de, const N: usize> serde::Deserialize<'de> for ConstMessage<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let formats = Vec::<MessageFormat>::deserialize(deserializer)?;
        for i in &formats {
            if let MessageFormat::Arg(n) = i {
                if *n >= N {
                    return Err(serde::de::Error::custom(format!(
                        "please set number between 0 and {} exclusive",
                        N
                    )));
                }
            }
        }

        Ok(Self(formats))
    }
}

impl<const N: usize> serde::Serialize for ConstMessage<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for MessageFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = MessageFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("allow only string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.starts_with('{') && v.ends_with('}') {
                    let number = v[1..v.len() - 1].parse::<usize>().map_err(|err| {
                        serde::de::Error::custom(format!("please set number: {}", err))
                    })?;
                    Ok(MessageFormat::Arg(number))
                } else {
                    Ok(MessageFormat::Text(v.to_string()))
                }
            }
        }
        deserializer.deserialize_string(Visitor)
    }
}

impl serde::Serialize for MessageFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageFormat::Text(text) => serializer.serialize_str(text),
            MessageFormat::Arg(num) => serializer.serialize_str(&format!("{{{}}}", num)),
        }
    }
}

mod macros {
    macro_rules! gen_message {
        (@gen $text:literal) => {
            MessageFormat::Text($text.to_string())
        };
        (@gen {$number:literal}) => {
            MessageFormat::Arg($number)
        };
        ($($tt:tt),*) => {
            ConstMessage(vec![$(gen_message!(@gen $tt)),*])
        }
    }

    pub(super) use gen_message;
}
