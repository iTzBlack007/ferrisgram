// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};


/// This object represents a forum topic.
/// <https://core.telegram.org/bots/api#forumtopic>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: i64,
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Optional. Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}