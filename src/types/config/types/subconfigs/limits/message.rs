// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MessageLimits {
    pub max_characters: u32,
    #[serde(default)]
    pub max_tts_characters: u32,
    pub max_reactions: u32,
    pub max_attachment_size: u64,
    pub max_bulk_delete: u32,
    pub max_embed_download_size: u64,
}

impl Default for MessageLimits {
    fn default() -> Self {
        Self {
            max_characters: 1048576,
            max_tts_characters: 160,
            max_reactions: 2048,
            max_attachment_size: 1024 * 1024 * 1024,
            max_bulk_delete: 1000,
            max_embed_download_size: 1024 * 1024 * 5,
        }
    }
}
