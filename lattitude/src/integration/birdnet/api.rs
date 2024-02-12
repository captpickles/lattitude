use chrono::{DateTime, Utc};
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Envelope {
    pub detections: Vec<Detection>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Detection {
    pub timestamp: DateTime<Utc>,
    pub species: Species,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub common_name: String,
    pub scientific_name: String,
}