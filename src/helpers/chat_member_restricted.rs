// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatMemberRestricted;
use crate::types::User;

impl ChatMemberRestricted {
    /// This function creates an empty struct for the object ChatMemberRestricted.
    pub fn new() -> Self {
        Self {
            user: User::new(),
            is_member: false,
            can_send_messages: false,
            can_send_audios: false,
            can_send_documents: false,
            can_send_photos: false,
            can_send_videos: false,
            can_send_video_notes: false,
            can_send_voice_notes: false,
            can_send_polls: false,
            can_send_other_messages: false,
            can_add_web_page_previews: false,
            can_change_info: false,
            can_invite_users: false,
            can_pin_messages: false,
            can_manage_topics: false,
            until_date: 0,
        }
    }
}
impl Default for ChatMemberRestricted {
    fn default() -> Self {
        Self::new()
    }
}