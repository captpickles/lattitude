use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Envelope {
    pub detections: Vec<Detection>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Detection {
   pub species: Species,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Species {
    pub common_name: String,
    pub scientific_name: String,
}