// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{InputFile, MaskPosition};
use serde::{Deserialize, Serialize};

/// This object describes a sticker to be added to a sticker set.
/// <https://core.telegram.org/bots/api#inputsticker>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputSticker {
    /// The added sticker. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. Animated and video stickers can't be uploaded via HTTP URL. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub sticker: InputFile,
    /// List of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
    /// Optional. Position where the mask should be placed on faces. For "mask" stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// Optional. List of 0-20 search keywords for the sticker with total length of up to 64 characters. For "regular" and "custom_emoji" stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}
