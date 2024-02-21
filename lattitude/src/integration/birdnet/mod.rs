mod api;

use ab_glyph::FontRef;
use actix::Message;
use chrono::{DateTime, Duration, Timelike, Utc};
use engine::controller::{Controller, Controllers};
use engine::integration::Integration;
use engine::view::canvas::Canvas;
use engine::view::text::FormattedText;
use engine::view::Renderable;
use pixelfield::pixelfield::PixelField;
use reqwest::{blocking, Client};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

const BASE_URL: &str = "https://app.birdweather.com/api/v1/stations";

pub enum BirdNetControllers {
    RecentDetections,
}

pub struct BirdNet {}

impl Integration for BirdNet {
    type Configuration = ();
    type Controllers = BirdNetControllers;

    fn create_controller(&self, controller: Self::Controllers) -> impl Controller {
        match controller {
            BirdNetControllers::RecentDetections => BirdNetRecentDetections::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub token: String,
    pub keep: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecentDetections {
    pub detections: Vec<api::Detection>,
}

#[derive(Default)]
pub struct BirdNetRecentDetections {
    configuration: Option<Configuration>,
    last_fetch: Option<DateTime<Utc>>,
    detections: VecDeque<api::Detection>,
}

impl BirdNetRecentDetections {
    pub fn new() -> Self {
        Self {
            configuration: None,
            last_fetch: None,
            detections: Default::default(),
        }
    }
}

impl Controller for BirdNetRecentDetections {
    type Integration = BirdNet;
    type Configuration = Configuration;
    type Output = RecentDetections;

    fn identifier(&self) -> String {
        "birdNET".to_string()
    }

    fn cadence(&self) -> Duration {
        Duration::minutes(10)
    }

    fn configure(&mut self, configuration: Option<Self::Configuration>) {
        println!("configure! {:?}", configuration);
        self.configuration = configuration
    }

    async fn update(&mut self) -> Option<Self::Output> {
        if let Some(configuration) = &self.configuration {
            println!("query birdnet");
            if let Ok(response) = Client::new()
                .get(format!("{}/{}/detections", BASE_URL, configuration.token))
                .query(&[(
                    "from".to_string(),
                    self.last_fetch
                        .map(|fetch| fetch.to_rfc3339())
                        .unwrap_or("".to_string()),
                )])
                .send()
                .await
            {
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

                    let mut num_short = if detections.len() < configuration.keep {
                        configuration.keep - detections.len()
                    } else {
                        0
                    };

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
            text: FormattedText::new(state, width, font, size, |recent: RecentDetections| {
                let names = recent
                    .detections
                    .iter()
                    .map(|e| {
                        let when = format!("{}:{}", e.timestamp.hour(), e.timestamp.minute());
                        format!("• {} {}", when, e.species.common_name)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                Some(names)
            }),
        }
    }
}

impl Renderable for BirdList {
    fn render<'r>(&'r self) -> Pin<Box<dyn Future<Output = Option<PixelField>> + 'r>> {
        self.text.render()
    }
}
