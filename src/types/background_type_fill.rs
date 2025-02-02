// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::BackgroundFill;
use serde::{Deserialize, Serialize};

/// The background is automatically filled based on the selected colors.
/// <https://core.telegram.org/bots/api#backgroundtypefill>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackgroundTypeFill {
    /// The background fill
    pub fill: BackgroundFill,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: i64,
}
