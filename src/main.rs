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

// TODO: I think I just need to set permissions for the bot
// Also add some logging
#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(
        &self,
        ctx: Context,
        opt_gid: Option<GuildId>,
        oldVcSt: Option<VoiceState>,
        st: VoiceState,
    ) {
        if let (Some(gid), Some(member), Some(channel)) = (opt_gid, st.member, st.channel_id) {
            if let Some(name) = channel.name(&ctx).await {
                if "OnlyCams (required)".to_string() == name && !st.self_video {
                    // TODO spawn a thread to check the status
                    // again in 3 seconds
                    sleep(Duration::from_secs(10)).await;
                    if let Some(guild) = gid.to_guild_cached(&ctx).await {
                        // lookup member.user.id in
                        // guild.voice_states
                        // then check if the video is still disabled
                        // if so then...
                        if let Some(new_st) = guild.voice_states.get(&member.user.id) {
                            if !new_st.self_video {
                                member.disconnect_from_voice(&ctx).await;
                            }
                        }
                    }
                }
            }
        }
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