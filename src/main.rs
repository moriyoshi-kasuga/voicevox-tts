use commands::ping::ping;
use config::init_config;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use songbird::SerenityInit;
use vvcore::*;

pub mod commands;
pub mod config;

type Error = Box<dyn std::error::Error + Send + Sync>;

struct Data {
    vvc: VoicevoxCore,
}

type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
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

    let vvc = init_vvc();

    let commands = vec![register(), ping()];

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
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { vvc })
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged();

    let client = ClientBuilder::new(config.discord_token, intents)
        .framework(framework)
        .register_songbird()
        .await;

    client.unwrap().start().await.unwrap()
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
