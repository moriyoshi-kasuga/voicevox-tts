use std::sync::Arc;

use commands::{join::join, leave::leave};
use config::{
    dictionary::{init_dictionary, Dictionary},
    init_config,
    messages::VoiceConfig,
};
use poise::serenity_prelude::{ClientBuilder, FullEvent, GatewayIntents};
use songbird::SerenityInit;
use util::{DictionaryKey, VoiceConfigKey, VoicevoxCoreKey};
use vvcore::*;

pub mod commands;
pub mod config;
pub mod event;
pub mod util;

pub type AnyError = Box<dyn std::error::Error + Send + Sync>;

pub type AnyResult<T> = Result<T, AnyError>;

pub struct Data {
    pub vvc: Arc<VoicevoxCore>,
    pub voice_config: Arc<VoiceConfig>,
    pub dictionary: Dictionary,
}

pub type Context<'a> = poise::Context<'a, Data, AnyError>;

const INTENTS: poise::serenity_prelude::GatewayIntents =
    GatewayIntents::non_privileged().union(GatewayIntents::MESSAGE_CONTENT);

#[poise::command(slash_command)]
async fn register(ctx: Context<'_>) -> Result<(), AnyError> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
#[allow(clippy::unwrap_used, clippy::panic)]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = init_config();

    if config.discord_token.is_empty() {
        panic!("please set discord token");
    };

    let voice_config = Arc::new(config.voices);
    let voice_config_clone = voice_config.clone();

    let dictionary = init_dictionary();
    let dict = dictionary.clone();

    let vvc = Arc::new(init_vvc());
    let vv_clone = vvc.clone();

    // COMMANDS
    let commands = vec![register(), join(), leave()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = poise::builtins::on_error(error).await {
                        tracing::error!("Fatal error while sending error message: {}", e);
                    }
                })
            },
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event {
                        FullEvent::VoiceStateUpdate { old, new } => {
                            event::voice_state::handle_voice_state_update(ctx, old, new).await?;
                        }
                        FullEvent::Message { new_message } => {
                            let _ = new_message;
                        }
                        _ => {}
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    vvc: Arc::clone(&vvc),
                    voice_config: Arc::clone(&voice_config).clone(),
                    dictionary,
                })
            })
        })
        .build();

    let mut client = ClientBuilder::new(config.discord_token, INTENTS)
        .framework(framework)
        .register_songbird()
        .await
        .unwrap();

    {
        let mut write = client.data.write().await;
        write.insert::<DictionaryKey>(dict.clone());
        write.insert::<VoicevoxCoreKey>(vv_clone);
        write.insert::<VoiceConfigKey>(voice_config_clone);
    }

    client.start().await.unwrap();

    dict.save().await;
}

#[allow(clippy::unwrap_used)]
fn init_vvc() -> VoicevoxCore {
    tracing::info!("loading vvc");

    let dir = std::ffi::CString::new("./voicevox_core/open_jtalk_dic_utf_8-1.11").unwrap();
    let vvc =
        VoicevoxCore::new_from_options(AccelerationMode::Auto, 0, true, dir.as_c_str()).unwrap();

    tracing::info!("loaded vvc");

    vvc
}
