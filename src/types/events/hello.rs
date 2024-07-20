// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::WebSocketEvent;
use chorus_macros::WebSocketEvent;
use serde::{Deserialize, Serialize};

/// Received on gateway init, tells the client how often to send heartbeats;
#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, WebSocketEvent, Copy, Hash, PartialOrd, Ord)]
pub struct GatewayHello {
    pub op: i32,
    pub d: HelloData,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, Copy, WebSocketEvent, Hash, PartialOrd, Ord)]
/// Contains info on how often the client should send heartbeats to the server;
pub struct HelloData {
    /// How often a client should send heartbeats, in milliseconds
    pub heartbeat_interval: u64,
}

