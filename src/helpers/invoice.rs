// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Invoice;

impl Invoice {
    /// This function creates an empty struct for the object Invoice.
    pub fn new(title: String, description: String, start_parameter: String, currency: String, total_amount: i64) -> Self {
        Self {
            title,
            description,
            start_parameter,
            currency,
            total_amount,
        }
    }
}