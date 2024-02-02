mod detections;

use std::time::Duration;
use actix::{Message};
use layout::controller::Controller;
use layout::controller::periodic::PeriodicController;
use serde::{Deserialize, Serialize};
use reqwest::blocking;
use crate::integration::birdnet::detections::{Detection, Envelope};

const BASE_URL: &str = "https://app.birdweather.com/api/v1/stations";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub token: String,
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct RecentDetections {
    pub detections: Vec<Detection>,
}

pub struct BirdNet {
    configuration: Option<Configuration>,
}


impl Controller for BirdNet {
    type Output = RecentDetections;
    type Configuration = Configuration;

    fn configure(&mut self, configuration: Option<Self::Configuration>) {
        self.configuration = configuration
    }
}

impl PeriodicController for BirdNet {
    fn cadence(&mut self) -> Duration {
        Duration::from_secs(60 * 5)
    }

    fn period_expired(&mut self) -> Option<Self::Output> {
        if let Some(configuration) = &self.configuration {
            if let Ok(response) = blocking::Client::new()
                .get(format!("{}/{}/detections", BASE_URL, configuration.token))
                .send() {
                if let Ok(data) = response.json::<Envelope>() {
                    return Some( RecentDetections {
                        detections: data.detections
                    } )
                }
            }
        }

        None
    }
}