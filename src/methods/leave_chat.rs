// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
    /// <https://core.telegram.org/bots/api#leavechat>
    pub fn leave_chat(&self, chat_id: i64) -> LeaveChatBuilder {
        LeaveChatBuilder::new(self, chat_id)
    }
}

#[derive(Serialize)]
pub struct LeaveChatBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i64,
}


impl<'a> LeaveChatBuilder<'a> {
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
        self.bot.get("leaveChat", Some(&form)).await
    }

}