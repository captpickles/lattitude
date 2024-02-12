mod api;

use actix::Message;
use chrono::{DateTime, Utc};
use reqwest::{blocking, Client};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::future::Future;
use std::time::Duration;
use engine::controller::Controller;

const BASE_URL: &str = "https://app.birdweather.com/api/v1/stations";

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub token: String,
    pub keep: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecentDetections {
    pub detections: Vec<api::Detection>,
}

pub struct BirdNet {
    configuration: Option<Configuration>,
    last_fetch: Option<DateTime<Utc>>,
    detections: VecDeque<api::Detection>,
}

impl Controller for BirdNet {
    type Output = RecentDetections;
    type Configuration = Configuration;

    fn configure(&mut self, configuration: Option<Self::Configuration>) {
        self.configuration = configuration
    }

    fn update(&mut self) -> impl Future<Output=Option<Self::Output>> + Send {
        async move {
            if let Some(configuration) = &self.configuration {
                if let Ok(response) = Client::new()
                    .get(format!("{}/{}/detections", BASE_URL, configuration.token))
                    .query(&[(
                        "from".to_string(),
                        self.last_fetch
                            .map(|fetch| fetch.to_rfc3339())
                            .unwrap_or("".to_string()),
                    )])
                    .send()
                    .await {
                    if let Ok(data) = response.json::<api::Envelope>().await {
                        let mut detections = Vec::new();

                        for detection in &data.detections {
                            if !detections
                                .iter()
                                .any(|e: &api::Detection| detection.species == e.species)
                            {
                                detections.push(detection.clone())
                            }
                        }

                        let mut num_short = configuration.keep - detections.len();

                        while num_short > 0 {
                            if let Some(backfill) = self.detections.pop_front() {
                                detections.push(backfill);
                                num_short -= 1;
                            } else {
                                break;
                            }
                        }

                        self.detections = detections.iter().cloned().collect();

                        return Some(RecentDetections { detections });
                    }
                }
            }

            None
        }
    }
}
    /*

impl PeriodicController for BirdNet {
    fn cadence(&mut self) -> Duration {
        Duration::from_secs(60 * 5)
    }

    fn period_expired(&mut self) -> Option<Self::Output> {
        if let Some(configuration) = &self.configuration {
            if let Ok(response) = blocking::Client::new()
                .get(format!("{}/{}/detections", BASE_URL, configuration.token))
                .query(&[(
                    "from".to_string(),
                    self.last_fetch
                        .map(|fetch| fetch.to_rfc3339())
                        .unwrap_or("".to_string()),
                )])
                .send()
            {
                if let Ok(data) = response.json::<api::Envelope>() {
                    let mut detections = Vec::new();

                    for detection in &data.detections {
                        if !detections
                            .iter()
                            .any(|e: &api::Detection| detection.species == e.species)
                        {
                            detections.push(detection.clone())
                        }
                    }

                    let mut num_short = configuration.keep - detections.len();

                    while num_short > 0 {
                        if let Some(backfill) = self.detections.pop_front() {
                            detections.push(backfill);
                            num_short -= 1;
                        } else {
                            break;
                        }
                    }

                    self.detections = detections.iter().cloned().collect();

                    return Some(RecentDetections { detections });
                }
            }
        }

        None
    }
}



     */