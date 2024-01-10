// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::{ReplyParameters, InlineKeyboardMarkup};
use crate::types::Message;

impl Bot {
    /// Use this method to send information about a venue. On success, the sent Message is returned.
    /// <https://core.telegram.org/bots/api#sendvenue>
    pub fn send_venue(&self, chat_id: i64, latitude: f64, longitude: f64, title: String, address: String) -> SendVenueBuilder {
        SendVenueBuilder::new(self, chat_id, latitude, longitude, title, address)
    }
}

#[derive(Serialize)]
pub struct SendVenueBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, "arts_entertainment/default", "arts_entertainment/aquarium" or "food/icecream".)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


impl<'a> SendVenueBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, latitude: f64, longitude: f64, title: String, address: String) -> Self {
        Self{
            bot,
            chat_id,
            message_thread_id: None,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;self
    }
                
    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);self
    }
                
    pub fn latitude(mut self, latitude: f64) -> Self {
        self.latitude = latitude;self
    }
                
    pub fn longitude(mut self, longitude: f64) -> Self {
        self.longitude = longitude;self
    }
                
    pub fn title(mut self, title: String) -> Self {
        self.title = title;self
    }
                
    pub fn address(mut self, address: String) -> Self {
        self.address = address;self
    }
                
    pub fn foursquare_id(mut self, foursquare_id: String) -> Self {
        self.foursquare_id = Some(foursquare_id);self
    }
                
    pub fn foursquare_type(mut self, foursquare_type: String) -> Self {
        self.foursquare_type = Some(foursquare_type);self
    }
                
    pub fn google_place_id(mut self, google_place_id: String) -> Self {
        self.google_place_id = Some(google_place_id);self
    }
                
    pub fn google_place_type(mut self, google_place_type: String) -> Self {
        self.google_place_type = Some(google_place_type);self
    }
                
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);self
    }
                
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);self
    }
                
    pub fn reply_parameters(mut self, reply_parameters: ReplyParameters) -> Self {
        self.reply_parameters = Some(reply_parameters);self
    }
                
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);self
    }
                
    pub async fn send(self) -> Result<Message> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("sendVenue", Some(&form)).await
    }

}