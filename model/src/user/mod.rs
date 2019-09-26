mod connection;
mod connection_visibility;
mod current_user;
mod flags;
mod premium_type;
mod profile;

pub use self::{
    connection::Connection,
    connection_visibility::ConnectionVisibility,
    current_user::CurrentUser,
    flags::UserFlags,
    premium_type::PremiumType,
    profile::UserProfile,
};

use crate::id::UserId;
use serde::{Deserialize, Serialize};
use serde_mappable_seq::Key;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    pub id: UserId,
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    pub discriminator: String,
    #[serde(rename = "username")]
    pub name: String,
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Key<'_, UserId> for User {
    fn key(&self) -> UserId {
        self.id
    }
}