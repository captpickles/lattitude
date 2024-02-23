
use std::hash::{Hash, Hasher};
use chrono::{DateTime, Local};
use serde::Deserialize;
use crate::integration::accuweather::api::{Ice, Rain, Snow, TotalLiquid, Wind};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Envelope {
    pub daily_forecasts: Vec<DailyForecast>,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DailyForecast {
    pub date: DateTime<Local>,
    pub sun: Sun,
    pub moon: Moon,
    pub temperature: Temperature,
    pub day: Details,
    pub night: Details,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Sun {
    pub rise: DateTime<Local>,
    pub set: DateTime<Local>,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Moon {
    pub rise: Option<DateTime<Local>>,
    pub set: Option<DateTime<Local>>,
    pub phase: String,
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Temperature {
    pub minimum: TempValue,
    pub maximum: TempValue,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TempValue {
    pub value: f32,
}

impl Hash for TempValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write( &self.value.to_be_bytes())
    }
}


#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Details {
    pub icon: u8,
    pub icon_phrase: String,
    pub short_phrase: String,
    pub long_phrase: String,
    pub precipitation_probability: u8,
    pub total_liquid: TotalLiquid,
    pub snow: Snow,
    pub rain: Rain,
    pub ice: Ice,
    pub wind: Wind,
    pub wind_gust: Wind,
}
