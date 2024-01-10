// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success.
    /// <https://core.telegram.org/bots/api#unbanchatsenderchat>
    pub fn unban_chat_sender_chat(&self, chat_id: i64, sender_chat_id: i64) -> UnbanChatSenderChatBuilder {
        UnbanChatSenderChatBuilder::new(self, chat_id, sender_chat_id)
    }
}

#[derive(Serialize)]
pub struct UnbanChatSenderChatBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}


impl<'a> UnbanChatSenderChatBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, sender_chat_id: i64) -> Self {
        Self{
            bot,
            chat_id,
            sender_chat_id,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;self
    }
                
    pub fn sender_chat_id(mut self, sender_chat_id: i64) -> Self {
        self.sender_chat_id = sender_chat_id;self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("unbanChatSenderChat", Some(&form)).await
    }

}