// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
    /// We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
    /// <https://core.telegram.org/bots/api#sendchataction>
    pub fn send_chat_action(&self, chat_id: i64, action: String) -> SendChatActionBuilder {
        SendChatActionBuilder::new(self, chat_id, action)
    }
}

#[derive(Serialize)]
pub struct SendChatActionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread; supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_voice or upload_voice for voice notes, upload_document for general files, choose_sticker for stickers, find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: String,
}


impl <'a> SendChatActionBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, action: String) -> Self {
        Self{
            bot,
            chat_id,
            message_thread_id: None,
            action,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
                
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
                
    pub fn action(mut self, action: String) -> Self {
        self.action = action;
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("sendChatAction", Some(&form)).await
    }

}