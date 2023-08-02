use serde::{Deserialize, Serialize};

use crate::types::utils::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct SecurityKey {
    pub id: String,
    pub user_id: String,
    pub key_id: String,
    pub public_key: String,
    pub counter: u64,
    pub name: String,
}

impl Default for SecurityKey {
    fn default() -> Self {
        Self {
            id: Snowflake::generate().to_string(),
            user_id: String::new(),
            key_id: String::new(),
            public_key: String::new(),
            counter: 0,
            name: String::new(),
        }
    }
}
