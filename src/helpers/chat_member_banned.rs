// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatMemberBanned;
use crate::types::User;

impl ChatMemberBanned {
    /// This function creates an empty struct for the object ChatMemberBanned.
    pub fn new(user: User, until_date: i64) -> Self {
        Self {
            user,
            until_date,
        }
    }
}