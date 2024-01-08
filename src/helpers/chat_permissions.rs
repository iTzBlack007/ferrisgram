// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatPermissions;

impl Default for ChatPermissions {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatPermissions {
    /// This function creates an empty struct for the object ChatPermissions.
    pub fn new() -> Self {
        Self {
            can_send_messages: None,
            can_send_audios: None,
            can_send_documents: None,
            can_send_photos: None,
            can_send_videos: None,
            can_send_video_notes: None,
            can_send_voice_notes: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
            can_manage_topics: None,
        }
    }
}
