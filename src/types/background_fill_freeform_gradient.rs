// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};

/// The background is a freeform gradient that rotates after every message in the chat.
/// <https://core.telegram.org/bots/api#backgroundfillfreeformgradient>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundFillFreeformGradient {
    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    pub colors: Vec<i64>,
}
