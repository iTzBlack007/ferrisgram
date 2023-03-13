// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the can_manage_topics administrator rights, unless it is the creator of the topic. Returns True on success.
    /// <https://core.telegram.org/bots/api#closeforumtopic>
    pub fn close_forum_topic(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> CloseForumTopicBuilder {
        CloseForumTopicBuilder::new(self, chat_id, message_thread_id)
    }
}

#[derive(Serialize)]
pub struct CloseForumTopicBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

impl<'a> CloseForumTopicBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, message_thread_id: i64) -> Self {
        Self {
            bot,
            chat_id,
            message_thread_id,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = message_thread_id;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("closeForumTopic", Some(&form)).await
    }
}
