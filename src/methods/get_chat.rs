// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::Chat;

impl Bot {
    /// Use this method to get up to date information about the chat. Returns a Chat object on success.
    /// <https://core.telegram.org/bots/api#getchat>
    pub fn get_chat(&self, chat_id: i64) -> GetChatBuilder {
        GetChatBuilder::new(self, chat_id)
    }
}

#[derive(Serialize)]
pub struct GetChatBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i64,
}


impl<'a> GetChatBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64) -> Self {
        Self{
            bot,
            chat_id,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;self
    }
                
    pub async fn send(self) -> Result<Chat> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("getChat", Some(&form)).await
    }

}