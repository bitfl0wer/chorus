// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus_macros::{JsonField, SourceUrlField};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::entities::{Guild, PublicUser, UnavailableGuild};
use crate::types::events::WebSocketEvent;
use crate::types::{
    AuditLogEntry, Emoji, GuildMember, GuildScheduledEvent, JsonField, RoleObject, Snowflake,
    SourceUrlField, Sticker,
};

use super::PresenceUpdate;

#[cfg(feature = "client")]
use super::UpdateMessage;
#[cfg(feature = "client")]
use crate::types::IntoShared;
#[cfg(feature = "client")]
use crate::types::Shared;

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Default,
    Clone,
    SourceUrlField,
    JsonField,
    WebSocketEvent,
    PartialEq,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-create>;
/// Received to give data about a guild;
// This one is particularly painful, it can be a Guild object with an extra field or an unavailable guild object
pub struct GuildCreate {
    #[serde(flatten)]
    pub d: GuildCreateDataOption,
    #[serde(skip)]
    pub source_url: String,
    #[serde(skip)]
    pub json: String,
}

#[cfg(feature = "client")]
#[cfg(not(tarpaulin_include))]
impl UpdateMessage<Guild> for GuildCreate {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        match &self.d {
            GuildCreateDataOption::UnavailableGuild(unavailable) => Some(unavailable.id),
            GuildCreateDataOption::Guild(guild) => Some(guild.id),
        }
    }

    fn update(&mut self, _: &mut Guild) {}
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum GuildCreateDataOption {
    UnavailableGuild(UnavailableGuild),
    Guild(Guild),
}

impl Default for GuildCreateDataOption {
    fn default() -> Self {
        GuildCreateDataOption::UnavailableGuild(UnavailableGuild::default())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuildEvents {
    Create(GuildCreate),
    Update(GuildUpdate),
    Delete(GuildDelete),
    BanAdd(GuildBanAdd),
    BanRemove(GuildBanRemove),
    EmojisUpdate(GuildEmojisUpdate),
    StickersUpdate(GuildStickersUpdate),
    IntegrationsUpdate(GuildIntegrationsUpdate),
    MemberAdd(GuildMemberAdd),
    MemberRemove(GuildMemberRemove),
    MemberUpdate(GuildMemberUpdate),
    MembersChunk(GuildMembersChunk),
    RoleCreate(GuildRoleCreate),
    RoleUpdate(GuildRoleUpdate),
    RoleDelete(GuildRoleDelete),
    ScheduledEventCreate(GuildScheduledEventCreate),
    ScheduledEventUpdate(GuildScheduledEventUpdate),
    ScheduledEventDelete(GuildScheduledEventDelete),
    ScheduledEventUserAdd(GuildScheduledEventUserAdd),
    ScheduledEventUserRemove(GuildScheduledEventUserRemove),
    AuditLogEntryCreate(GuildAuditLogEntryCreate),
}
#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-ban-add-guild-ban-add-event-fields>;
/// Received to give info about a user being banned from a guild;
pub struct GuildBanAdd {
    pub guild_id: Snowflake,
    pub user: PublicUser,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-ban-remove>;
/// Received to give info about a user being unbanned from a guild;
pub struct GuildBanRemove {
    pub guild_id: Snowflake,
    pub user: PublicUser,
}

#[derive(
    Debug,
    Default,
    Deserialize,
    Serialize,
    Clone,
    SourceUrlField,
    JsonField,
    WebSocketEvent,
    PartialEq,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-update>;
/// Received to give info about a guild being updated;
pub struct GuildUpdate {
    #[serde(flatten)]
    pub guild: Guild,
    #[serde(skip)]
    pub source_url: String,
    #[serde(skip)]
    pub json: String,
}

#[cfg(feature = "client")]
impl UpdateMessage<Guild> for GuildUpdate {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        Some(self.guild.id)
    }
}

#[derive(
    Debug,
    Default,
    Deserialize,
    Serialize,
    Clone,
    SourceUrlField,
    JsonField,
    WebSocketEvent,
    PartialEq,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-delete>;
/// Received to tell the client about a guild being deleted;
pub struct GuildDelete {
    #[serde(flatten)]
    pub guild: UnavailableGuild,
    #[serde(skip)]
    pub source_url: String,
    #[serde(skip)]
    pub json: String,
}

#[cfg(feature = "client")]
impl UpdateMessage<Guild> for GuildDelete {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        Some(self.guild.id)
    }
    fn update(&mut self, _: &mut Guild) {}
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-audit-log-entry-create>;
/// Received to the client about an audit log entry being added;
pub struct GuildAuditLogEntryCreate {
    #[serde(flatten)]
    pub entry: AuditLogEntry,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-emojis-update>;
/// Received to tell the client about a change to a guild's emoji list;
pub struct GuildEmojisUpdate {
    pub guild_id: Snowflake,
    pub emojis: Vec<Emoji>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-stickers-update>;
/// Received to tell the client about a change to a guild's sticker list;
pub struct GuildStickersUpdate {
    pub guild_id: Snowflake,
    pub stickers: Vec<Sticker>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq, Copy, Eq, Hash, PartialOrd, Ord)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-integrations-update>
pub struct GuildIntegrationsUpdate {
    pub guild_id: Snowflake,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-member-add>;
/// Received to tell the client about a user joining a guild;
pub struct GuildMemberAdd {
    #[serde(flatten)]
    pub member: GuildMember,
    pub guild_id: Snowflake,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-member-remove>;
/// Received to tell the client about a user leaving a guild;
pub struct GuildMemberRemove {
    pub guild_id: Snowflake,
    pub user: PublicUser,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-member-update>
pub struct GuildMemberUpdate {
    pub guild_id: Snowflake,
    pub roles: Vec<Snowflake>,
    pub user: PublicUser,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub joined_at: Option<DateTime<Utc>>,
    pub premium_since: Option<DateTime<Utc>>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    pub communication_disabled_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-members-chunk>
pub struct GuildMembersChunk {
    pub guild_id: Snowflake,
    pub members: Vec<GuildMember>,
    pub chunk_index: u16,
    pub chunk_count: u16,
    pub not_found: Option<Vec<Snowflake>>,
    pub presences: Option<Vec<PresenceUpdate>>,
    pub nonce: Option<String>,
}

#[derive(
    Debug,
    Default,
    Deserialize,
    Serialize,
    Clone,
    JsonField,
    SourceUrlField,
    WebSocketEvent,
    PartialEq,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-role-create>
pub struct GuildRoleCreate {
    pub guild_id: Snowflake,
    pub role: RoleObject,
    #[serde(skip)]
    pub json: String,
    #[serde(skip)]
    pub source_url: String,
}

#[cfg(feature = "client")]
impl UpdateMessage<Guild> for GuildRoleCreate {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        Some(self.guild_id)
    }

    fn update(&mut self, write: &mut Guild) {
        write.roles.push(self.role.clone().into_shared());
    }
}

#[derive(
    Debug,
    Default,
    Deserialize,
    Serialize,
    Clone,
    JsonField,
    SourceUrlField,
    WebSocketEvent,
    PartialEq,
)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-role-update>
pub struct GuildRoleUpdate {
    pub guild_id: Snowflake,
    pub role: RoleObject,
    #[serde(skip)]
    pub json: String,
    #[serde(skip)]
    pub source_url: String,
}

#[cfg(feature = "client")]
impl UpdateMessage<RoleObject> for GuildRoleUpdate {
    #[cfg(not(tarpaulin_include))]
    fn id(&self) -> Option<Snowflake> {
        Some(self.role.id)
    }

    fn update(&mut self, write: &mut RoleObject) {
        *write = self.role.clone();
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-role-delete>
pub struct GuildRoleDelete {
    pub guild_id: Snowflake,
    pub role_id: Snowflake,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-create>
pub struct GuildScheduledEventCreate {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-update>
pub struct GuildScheduledEventUpdate {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-delete>
pub struct GuildScheduledEventDelete {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq, Copy, Eq, Hash, PartialOrd, Ord)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-add>
pub struct GuildScheduledEventUserAdd {
    pub guild_scheduled_event_id: Snowflake,
    pub user_id: Snowflake,
    pub guild_id: Snowflake,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, WebSocketEvent, PartialEq, Copy, Eq, Hash, PartialOrd, Ord)]
/// See <https://discord.com/developers/docs/topics/gateway-events#guild-scheduled-event-user-remove>
pub struct GuildScheduledEventUserRemove {
    pub guild_scheduled_event_id: Snowflake,
    pub user_id: Snowflake,
    pub guild_id: Snowflake,
}
