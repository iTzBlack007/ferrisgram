// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::SharedUser;

impl SharedUser {
    /// This function creates an empty struct for the object SharedUser.
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            first_name: None,
            last_name: None,
            username: None,
            photo: None,
        }
    }
}
