// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::TextQuote;

impl TextQuote {
    /// This function creates an empty struct for the object TextQuote.
    pub fn new(text: String, position: i64) -> Self {
        Self {
            text,
            entities: None,
            position,
            is_manual: None,
        }
    }
}