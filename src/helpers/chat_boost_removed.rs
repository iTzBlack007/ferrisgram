// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatBoostRemoved;
use crate::types::Chat;
use crate::types::ChatBoostSource;

impl ChatBoostRemoved {
    /// This function creates an empty struct for the object ChatBoostRemoved.
    pub fn new(chat: Chat, boost_id: String, remove_date: i64, source: ChatBoostSource) -> Self {
        Self {
            chat,
            boost_id,
            remove_date,
            source,
        }
    }
}