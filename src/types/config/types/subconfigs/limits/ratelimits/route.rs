// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::config::types::subconfigs::limits::ratelimits::{
    auth::AuthRateLimit, RateLimitOptions,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Copy, PartialOrd, Ord)]
pub struct RouteRateLimit {
    pub guild: RateLimitOptions,
    pub webhook: RateLimitOptions,
    pub channel: RateLimitOptions,
    pub auth: AuthRateLimit,
}

impl Default for RouteRateLimit {
    fn default() -> Self {
        Self {
            guild: RateLimitOptions {
                bot: None,
                count: 5,
                window: 5,
                only_ip: false,
            },
            webhook: RateLimitOptions {
                bot: None,
                count: 10,
                window: 5,
                only_ip: false,
            },
            channel: RateLimitOptions {
                bot: None,
                count: 10,
                window: 5,
                only_ip: false,
            },
            auth: AuthRateLimit::default(),
        }
    }
}
