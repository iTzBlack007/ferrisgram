// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::PollAnswer;

impl PollAnswer {
    /// This function creates an empty struct for the object PollAnswer.
    pub fn new(poll_id: String, option_ids: Vec<i64>) -> Self {
        Self {
            poll_id,
            voter_chat: None,
            user: None,
            option_ids,
        }
    }
}