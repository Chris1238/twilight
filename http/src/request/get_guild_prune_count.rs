use super::prelude::*;
use dawn_model::{guild::GuildPrune, id::GuildId};

pub struct GetGuildPruneCount<'a> {
    days: Option<u64>,
    fut: Option<Pin<Box<dyn Future<Output = Result<GuildPrune>> + Send + 'a>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildPruneCount<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: impl Into<GuildId>) -> Self {
        Self {
            days: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
        }
    }

    pub fn days(mut self, days: u64) -> Self {
        self.days.replace(days);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildPruneCount {
                days: self.days,
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildPruneCount<'_>, GuildPrune);