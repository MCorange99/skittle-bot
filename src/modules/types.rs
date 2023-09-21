use std::collections::HashMap;
use futures::future::BoxFuture;
use color_eyre::Result;
use serenity::{
    model::prelude::{
        *,
        automod::{
            ActionExecution,
            Rule
        },
        command::CommandPermission
    },
    client::bridge::gateway::event::ShardStageUpdateEvent,
    json::Value, http::ratelimiting::RatelimitInfo, prelude::Context
};

pub type SkittleModuleCommandFn      = fn(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>;
pub type SkittleModuleEventManagerFn = fn(ctx: Context, event: EventType)                -> BoxFuture<'static, Result<()>>;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum EventType {
    application_command_permissions_update(CommandPermission),
    auto_moderation_rule_create(Rule),
    auto_moderation_rule_update(Rule),
    auto_moderation_rule_delete(Rule),
    auto_moderation_action_execution(ActionExecution),
    cache_ready(Vec<GuildId>),
    channel_create(GuildChannel),
    category_create(ChannelCategory),
    category_delete(ChannelCategory),
    channel_delete(GuildChannel),
    channel_pins_update(ChannelPinsUpdateEvent),
    channel_update(Channel),
    guild_ban_addition(User),
    guild_ban_removal(User),
    guild_create(bool),
    guild_delete(Option<Guild>),
    guild_emojis_update(HashMap<EmojiId, Emoji>),
    guild_integrations_update(GuildId),
    guild_member_addition(Member),
    guild_member_removal(Option<Member>),
    guild_member_update(Member),
    guild_members_chunk(GuildMembersChunkEvent),
    guild_role_create(Role),
    guild_role_delete(Option<Role>),
    guild_role_update(Role),
    guild_stickers_update(HashMap<StickerId, Sticker>),
    guild_unavailable(GuildId),
    guild_update(PartialGuild),
    invite_create(InviteCreateEvent),
    invite_delete(InviteDeleteEvent),
    message(Message),
    message_delete(Option<GuildId>),
    message_delete_bulk(Option<GuildId>),
    message_update(MessageUpdateEvent),
    reaction_add(Reaction),
    reaction_remove(Reaction),
    reaction_remove_all(MessageId),
    presence_replace(Vec<Presence>),
    presence_update(Presence),
    ready(Ready),
    resume(ResumedEvent),
    shard_stage_update(ShardStageUpdateEvent),
    typing_start(TypingStartEvent),
    unknown(Value),
    user_update(CurrentUser),
    voice_server_update(VoiceServerUpdateEvent),
    voice_state_update(VoiceState),
    webhook_update(ChannelId),
    interaction_create(Interaction),
    integration_create(Integration),
    integration_update(Integration),
    integration_delete(Option<ApplicationId>),
    stage_instance_create(StageInstance),
    stage_instance_update(StageInstance),
    stage_instance_delete(StageInstance),
    thread_create(GuildChannel),
    thread_update(GuildChannel),
    thread_delete(PartialGuildChannel),
    thread_list_sync(ThreadListSyncEvent),
    thread_member_update(ThreadMember),
    thread_members_update(ThreadMembersUpdateEvent),
    guild_scheduled_event_create(ScheduledEvent),
    guild_scheduled_event_update(ScheduledEvent),
    guild_scheduled_event_delete(ScheduledEvent),
    guild_scheduled_event_user_add(GuildScheduledEventUserAddEvent),
    guild_scheduled_event_user_remove(GuildScheduledEventUserRemoveEvent),
    ratelimit(RatelimitInfo),
}