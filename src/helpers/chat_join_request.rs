// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Chat;
use crate::types::ChatJoinRequest;
use crate::types::User;

impl ChatJoinRequest {
    /// This function creates an empty struct for the object ChatJoinRequest.
    pub fn new() -> Self {
        Self {
            chat: Chat::new(),
            from: User::new(),
            user_chat_id: 0,
            date: 0,
            bio: None,
            invite_link: None,
        }
    }
}
impl Default for ChatJoinRequest {
    fn default() -> Self {
        Self::new()
    }
}
