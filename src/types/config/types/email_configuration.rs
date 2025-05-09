// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

use crate::types::config::types::subconfigs::email::{
    mailgun::MailGunConfiguration, mailjet::MailJetConfiguration, sendgrid::SendGridConfiguration,
    smtp::SMTPConfiguration,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum EmailProvider {
    Smtp,
    MailGun,
    MailJet,
    SendGrid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct EmailConfiguration {
    pub provider: Option<EmailProvider>,
    pub smtp: SMTPConfiguration,
    pub mailgun: MailGunConfiguration,
    pub mailjet: MailJetConfiguration,
    pub sendgrid: SendGridConfiguration,
}
