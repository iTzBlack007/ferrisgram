// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatMemberRestricted;
use crate::types::User;

impl ChatMemberRestricted {
    /// This function creates an empty struct for the object ChatMemberRestricted.
    pub fn new(user: User, is_member: bool, can_send_messages: bool, can_send_audios: bool, can_send_documents: bool, can_send_photos: bool, can_send_videos: bool, can_send_video_notes: bool, can_send_voice_notes: bool, can_send_polls: bool, can_send_other_messages: bool, can_add_web_page_previews: bool, can_change_info: bool, can_invite_users: bool, can_pin_messages: bool, can_manage_topics: bool, until_date: i64) -> Self {
        Self {
            user,
            is_member,
            can_send_messages,
            can_send_audios,
            can_send_documents,
            can_send_photos,
            can_send_videos,
            can_send_video_notes,
            can_send_voice_notes,
            can_send_polls,
            can_send_other_messages,
            can_add_web_page_previews,
            can_change_info,
            can_invite_users,
            can_pin_messages,
            can_manage_topics,
            until_date,
        }
    }
}