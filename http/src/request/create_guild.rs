use super::prelude::*;
use dawn_model::{
    channel::GuildChannel,
    guild::{
        DefaultMessageNotificationLevel,
        ExplicitContentFilter,
        PartialGuild,
        Role,
        VerificationLevel,
    },
};
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateGuild<'a> {
    channels: Option<Vec<GuildChannel>>,
    default_message_notifications: Option<DefaultMessageNotificationLevel>,
    explicit_content_filter: Option<ExplicitContentFilter>,
    icon: Option<String>,
    region: Option<String>,
    roles: Option<Vec<Role>>,
    verification_level: Option<VerificationLevel>,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<PartialGuild>> + Send + 'a>>>,
    #[serde(skip)]
    http: &'a Client,
    name: String,
}

impl<'a> CreateGuild<'a> {
    pub(crate) fn new(http: &'a Client, name: impl Into<String>) -> Self {
        Self {
            channels: None,
            default_message_notifications: None,
            explicit_content_filter: None,
            fut: None,
            http,
            icon: None,
            name: name.into(),
            region: None,
            roles: None,
            verification_level: None,
        }
    }

    pub fn channels(mut self, channels: Vec<GuildChannel>) -> Self {
        self.channels.replace(channels);

        self
    }

    pub fn default_message_notifications(
        mut self,
        default_message_notifications: DefaultMessageNotificationLevel,
    ) -> Self {
        self.default_message_notifications
            .replace(default_message_notifications);

        self
    }

    pub fn explicit_content_filter(
        mut self,
        explicit_content_filter: ExplicitContentFilter,
    ) -> Self {
        self.explicit_content_filter
            .replace(explicit_content_filter);

        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon.replace(icon.into());

        self
    }

    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region.replace(region.into());

        self
    }

    pub fn roles(mut self, roles: Vec<Role>) -> Self {
        self.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::CreateGuild,
        )))));

        Ok(())
    }
}

poll_req!(CreateGuild<'_>, PartialGuild);