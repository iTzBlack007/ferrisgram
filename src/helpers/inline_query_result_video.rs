// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultVideo;

impl InlineQueryResultVideo {
    /// This function creates an empty struct for the object InlineQueryResultVideo.
    pub fn new() -> Self {
        Self {
            r#type: "".to_string(),
            id: "".to_string(),
            video_url: "".to_string(),
            mime_type: "".to_string(),
            thumbnail_url: "".to_string(),
            title: "".to_string(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
impl Default for InlineQueryResultVideo {
    fn default() -> Self {
        Self::new()
    }
}
