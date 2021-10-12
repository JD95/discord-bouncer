use serenity::async_trait;
use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready, id::GuildId, voice::VoiceState};
use serenity::prelude::{Context, EventHandler};
use std::env;
use std::fmt::Display;
use std::time::{Duration, Instant};
use tokio::time::sleep;

struct Handler;

async fn handle_voice(ctx: Context, opt_gid: Option<GuildId>, st: VoiceState) -> Option<()> {
    let gid = opt_gid?;
    let member = st.member?;
    let channel = st.channel_id?;
    let name = channel.name(&ctx).await?;
    if "OnlyCams (required)".to_string() == name && !st.self_video {
        if let Some(roles) = member.roles(&ctx).await {
            if let Some(_r) = roles.iter().filter(|r| r.name == "No Bounce").next() {
                return Some(());
            }
        }

        sleep(Duration::from_secs(10)).await;
        let guild = gid.to_guild_cached(&ctx).await?;
        let new_st = guild.voice_states.get(&member.user.id)?;
        if !new_st.self_video {
            let _ = member.disconnect_from_voice(&ctx).await;
        }
    }

    Some(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(
        &self,
        ctx: Context,
        opt_gid: Option<GuildId>,
        old_vc_St: Option<VoiceState>,
        st: VoiceState,
    ) {
        let _ = handle_voice(ctx, opt_gid, st).await;
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = &env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
