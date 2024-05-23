use anyhow::Error;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub role: Uuid,
    pub active: bool,
    pub mfa_enabled: bool,
    pub mfa_verified: bool,
    pub mfa_secret: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub async fn is_system_admin(&self) -> Result<bool, Error> {
        let config: Config = Config::init();

        Ok(self.email == config.admin_email)
    }
}
