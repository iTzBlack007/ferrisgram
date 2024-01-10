// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to change the bot's name. Returns True on success.
    /// <https://core.telegram.org/bots/api#setmyname>
    pub fn set_my_name(&self) -> SetMyNameBuilder {
        SetMyNameBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct SetMyNameBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// New bot name; 0-64 characters. Pass an empty string to remove the dedicated name for the given language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose language there is no dedicated name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}


impl<'a> SetMyNameBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self{
            bot,
            name: None,
            language_code: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);self
    }
                
    pub fn language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("setMyName", Some(&form)).await
    }

}