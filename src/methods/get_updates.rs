// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::Update;

impl Bot {
    /// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
    /// <https://core.telegram.org/bots/api#getupdates>
    pub fn get_updates(&self) -> GetUpdatesBuilder {
        GetUpdatesBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct GetUpdatesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will be forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify ["message", "edited_channel_post", "callback_query"] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member, message_reaction, and message_reaction_count (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}


impl<'a> GetUpdatesBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self{
            bot,
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);self
    }
                
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);self
    }
                
    pub fn timeout(mut self, timeout: i64) -> Self {
        self.timeout = Some(timeout);self
    }
                
    pub fn allowed_updates(mut self, allowed_updates: Vec<String>) -> Self {
        self.allowed_updates = Some(allowed_updates);self
    }
                
    pub async fn send(self) -> Result<Vec<Update>> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("getUpdates", Some(&form)).await
    }

}