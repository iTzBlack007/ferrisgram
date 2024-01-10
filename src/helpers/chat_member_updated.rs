// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatMemberUpdated;
use crate::types::Chat;
use crate::types::User;
use crate::types::ChatMember;

impl ChatMemberUpdated {
    /// This function creates an empty struct for the object ChatMemberUpdated.
    pub fn new(chat: Chat, from: User, date: i64, old_chat_member: ChatMember, new_chat_member: ChatMember) -> Self {
        Self {
            chat,
            from,
            date,
            old_chat_member,
            new_chat_member,
            invite_link: None,
            via_chat_folder_invite_link: None,
        }
    }
}