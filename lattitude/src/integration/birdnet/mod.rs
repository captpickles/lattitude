mod api;

use actix::Message;
use chrono::{DateTime, Duration, Timelike, Utc};
use reqwest::{blocking, Client};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use ab_glyph::FontRef;
use tokio::sync::Mutex;
use engine::controller::Controller;
use engine::view::canvas::Canvas;
use engine::view::Renderable;
use engine::view::text::FormattedText;
use pixelfield::pixelfield::PixelField;

const BASE_URL: &str = "https://app.birdweather.com/api/v1/stations";

#[derive(Debug, Serialize, Deserialize)]
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

impl BirdNet {
    pub fn new() -> Self {
        Self {
            configuration: None,
            last_fetch: None,
            detections: Default::default(),
        }
    }
}

impl Controller for BirdNet {
    type Output = RecentDetections;
    type Configuration = Configuration;

    fn configure(&mut self, configuration: Option<Self::Configuration>) {
        println!("configure! {:?}", configuration);
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

    fn cadence(&self) -> Option<Duration> {
        Some(Duration::minutes(10))
    }

    fn identifier(&self) -> String {
        "birdNET".to_string()
    }
}

pub struct BirdList {
    text: FormattedText<RecentDetections, RecentDetections>,
}

impl BirdList {

    pub fn new(
        state: Arc<Mutex<Option<RecentDetections>>>,
        width: u32,
        font: FontRef<'static>,
        size: f32,
    ) -> Self {
        Self {
            text: FormattedText::new(
                state,
                width,
                font,
                size,
                |recent: RecentDetections| {
                    let names = recent.detections.iter().map(|e| {

                        let when = format!("{}:{}", e.timestamp.hour(), e.timestamp.minute());
                        format!("• {} {}", when, e.species.common_name)
                    }).collect::<Vec<_>>().join("\n");

                    Some(names)
                }
            )
        }
    }
}

impl Renderable for BirdList {
    fn render<'r>(&'r self) -> Pin<Box<dyn Future<Output=Option<PixelField>> + 'r>> {
        self.text.render()
    }
}