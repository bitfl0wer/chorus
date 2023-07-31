use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    entities::{Guild, GuildMember},
    utils::Snowflake,
};

/// See <https://docs.spacebar.chat/routes/#cmp--schemas-voicestate>
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct VoiceState {
    pub guild_id: Option<Snowflake>,
    pub guild: Option<Guild>,
    pub channel_id: Option<Snowflake>,
    pub user_id: Snowflake,
    pub member: Option<GuildMember>,
    pub session_id: Snowflake,
    pub token: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub self_video: bool,
    pub suppress: bool,
    pub request_to_speak_timestamp: Option<DateTime<Utc>>,
    pub id: Option<Snowflake>,
}
