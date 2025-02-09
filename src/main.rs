use std::sync::Arc;

use cache::{TtsChannel, TtsChannelKey};
use commands::{join::join, leave::leave};
use config::{dictionary::init_dictionary, init_config};
use poise::serenity_prelude::{ClientBuilder, FullEvent, GatewayIntents};
use songbird::SerenityInit;
use util::{DictionaryKey, VoiceConfigKey, VoicevoxCoreKey};
use vvcore::*;

pub mod cache;
pub mod commands;
pub mod config;
pub mod event;
pub mod util;

pub type AnyError = Box<dyn std::error::Error + Send + Sync>;

pub type AnyResult<T> = Result<T, AnyError>;

pub type Context<'a> = poise::Context<'a, (), AnyError>;

const INTENTS: poise::serenity_prelude::GatewayIntents =
    GatewayIntents::non_privileged().union(GatewayIntents::MESSAGE_CONTENT);

#[poise::command(slash_command)]
async fn register(ctx: Context<'_>) -> Result<(), AnyError> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

#[tokio::main]
#[allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
async fn main() {
    tracing_subscriber::fmt::init();

    let _ = dotenvy::dotenv();
    let discord_token = std::env::var("DISCORD_TOKEN").expect("please set discord token in env");

    let config = init_config();

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
                            event::message::handle_message(ctx, new_message).await?;
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
                Ok(())
            })
        })
        .build();

    let mut client = ClientBuilder::new(discord_token, INTENTS)
        .framework(framework)
        .register_songbird()
        .await
        .unwrap();

    {
        let mut write = client.data.write().await;
        write.insert::<DictionaryKey>(dict.clone());
        write.insert::<VoicevoxCoreKey>(vv_clone);
        write.insert::<VoiceConfigKey>(voice_config_clone);
        write.insert::<TtsChannelKey>(TtsChannel::default());
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
