use serde_inline_default::serde_inline_default;

use macros::gen_message;

use super::DefaultConfig;

pub type Message = Vec<MessageFormat>;

#[serde_inline_default]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MessagesConfig {
    #[serde_inline_default(gen_message!({0},"さんが参加しました"))]
    pub join: Message,
    #[serde_inline_default(gen_message!({0},"さんが退出しました"))]
    pub leave: Message,
}

impl DefaultConfig for MessagesConfig {}

#[derive(Debug)]
pub enum MessageFormat {
    Text(String),
    Arg(u8),
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
                    let number = v[1..v.len() - 1].parse::<u8>().map_err(|err| {
                        serde::de::Error::custom(format!(
                            "please set number between 0 ~ 255: {}",
                            err
                        ))
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
            vec![$(gen_message!(@gen $tt)),*]
        }
    }

    pub(super) use gen_message;
}
