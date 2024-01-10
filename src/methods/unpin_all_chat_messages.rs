// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
    /// <https://core.telegram.org/bots/api#unpinallchatmessages>
    pub fn unpin_all_chat_messages(&self, chat_id: i64) -> UnpinAllChatMessagesBuilder {
        UnpinAllChatMessagesBuilder::new(self, chat_id)
    }
}

#[derive(Serialize)]
pub struct UnpinAllChatMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
}


impl<'a> UnpinAllChatMessagesBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64) -> Self {
        Self{
            bot,
            chat_id,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("unpinAllChatMessages", Some(&form)).await
    }

}