// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::LabeledPrice;

impl Bot {
    /// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
    /// <https://core.telegram.org/bots/api#createinvoicelink>
    pub fn create_invoice_link(&self, title: String, description: String, payload: String, provider_token: String, currency: String, prices: Vec<LabeledPrice>) -> CreateInvoiceLinkBuilder {
        CreateInvoiceLinkBuilder::new(self, title, description, payload, provider_token, currency, prices)
    }
}

#[derive(Serialize)]
pub struct CreateInvoiceLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via BotFather
    pub provider_token: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Pass True if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass True if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass True if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass True if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass True if the user's phone number should be sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True if the user's email address should be sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass True if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}


impl<'a> CreateInvoiceLinkBuilder<'a> {
    pub fn new(bot: &'a Bot, title: String, description: String, payload: String, provider_token: String, currency: String, prices: Vec<LabeledPrice>) -> Self {
        Self{
            bot,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;self
    }
                
    pub fn description(mut self, description: String) -> Self {
        self.description = description;self
    }
                
    pub fn payload(mut self, payload: String) -> Self {
        self.payload = payload;self
    }
                
    pub fn provider_token(mut self, provider_token: String) -> Self {
        self.provider_token = provider_token;self
    }
                
    pub fn currency(mut self, currency: String) -> Self {
        self.currency = currency;self
    }
                
    pub fn prices(mut self, prices: Vec<LabeledPrice>) -> Self {
        self.prices = prices;self
    }
                
    pub fn max_tip_amount(mut self, max_tip_amount: i64) -> Self {
        self.max_tip_amount = Some(max_tip_amount);self
    }
                
    pub fn suggested_tip_amounts(mut self, suggested_tip_amounts: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(suggested_tip_amounts);self
    }
                
    pub fn provider_data(mut self, provider_data: String) -> Self {
        self.provider_data = Some(provider_data);self
    }
                
    pub fn photo_url(mut self, photo_url: String) -> Self {
        self.photo_url = Some(photo_url);self
    }
                
    pub fn photo_size(mut self, photo_size: i64) -> Self {
        self.photo_size = Some(photo_size);self
    }
                
    pub fn photo_width(mut self, photo_width: i64) -> Self {
        self.photo_width = Some(photo_width);self
    }
                
    pub fn photo_height(mut self, photo_height: i64) -> Self {
        self.photo_height = Some(photo_height);self
    }
                
    pub fn need_name(mut self, need_name: bool) -> Self {
        self.need_name = Some(need_name);self
    }
                
    pub fn need_phone_number(mut self, need_phone_number: bool) -> Self {
        self.need_phone_number = Some(need_phone_number);self
    }
                
    pub fn need_email(mut self, need_email: bool) -> Self {
        self.need_email = Some(need_email);self
    }
                
    pub fn need_shipping_address(mut self, need_shipping_address: bool) -> Self {
        self.need_shipping_address = Some(need_shipping_address);self
    }
                
    pub fn send_phone_number_to_provider(mut self, send_phone_number_to_provider: bool) -> Self {
        self.send_phone_number_to_provider = Some(send_phone_number_to_provider);self
    }
                
    pub fn send_email_to_provider(mut self, send_email_to_provider: bool) -> Self {
        self.send_email_to_provider = Some(send_email_to_provider);self
    }
                
    pub fn is_flexible(mut self, is_flexible: bool) -> Self {
        self.is_flexible = Some(is_flexible);self
    }
                
    pub async fn send(self) -> Result<String> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("createInvoiceLink", Some(&form)).await
    }

}