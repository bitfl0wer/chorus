// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd, Copy, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MetricsConfiguration {
    pub timeout: u64,
}

impl Default for MetricsConfiguration {
    fn default() -> Self {
        Self { timeout: 30000 }
    }
}
