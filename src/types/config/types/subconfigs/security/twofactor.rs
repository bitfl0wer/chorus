// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct TwoFactorConfiguration {
    pub generate_backup_codes: bool,
}

impl Default for TwoFactorConfiguration {
    fn default() -> Self {
        Self {
            generate_backup_codes: true,
        }
    }
}
