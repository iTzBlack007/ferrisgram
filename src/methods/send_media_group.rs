// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::{InputMediaAudio, ReplyParameters};
use crate::types::Message;

impl Bot {
    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
    /// <https://core.telegram.org/bots/api#sendmediagroup>
    pub fn send_media_group(&self, chat_id: i64, media: Vec<InputMediaAudio>) -> SendMediaGroupBuilder {
        SendMediaGroupBuilder::new(self, chat_id, media)
    }
}

#[derive(Serialize)]
pub struct SendMediaGroupBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMediaAudio>,
    /// Sends messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}


impl<'a> SendMediaGroupBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, media: Vec<InputMediaAudio>) -> Self {
        Self{
            bot,
            chat_id,
            message_thread_id: None,
            media,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;self
    }
                
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);self
    }
                
    pub fn media(mut self, media: Vec<InputMediaAudio>) -> Self {
        self.media = media;self
    }
                
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);self
    }
                
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);self
    }
                
    pub fn reply_parameters(mut self, reply_parameters: ReplyParameters) -> Self {
        self.reply_parameters = Some(reply_parameters);self
    }
                
    pub async fn send(self) -> Result<Vec<Message>> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("sendMediaGroup", Some(&form)).await
    }

}