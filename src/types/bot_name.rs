// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};


/// This object represents the bot's name.
/// <https://core.telegram.org/bots/api#botname>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotName {
    /// The bot's name
    pub name: String,
}