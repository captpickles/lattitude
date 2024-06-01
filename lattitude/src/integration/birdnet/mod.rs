mod api;

use ab_glyph::FontRef;
use actix::Message;
use chrono::{DateTime, Duration, Timelike, Utc};
use engine::engine::integrations::IntegrationContext;
use engine::global_configuration::GlobalConfiguration;
use engine::integration::{Integration, IntegrationInfo};
use engine::model::{ModelKey, ModelManager};
use engine::view::canvas::Canvas;
use engine::view::text::FormattedText;
use engine::view::Renderable;
use pixelfield::pixelfield::PixelField;
use reqwest::{blocking, Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use bevy_app::{App, FixedUpdate, Plugin, Startup};
use bevy_ecs::prelude::{Commands, Component, Entity, IntoSystemConfigs, Query};
use bevy_ecs::system::Res;
use bevy_tasks::{AsyncComputeTaskPool, FakeTask, Task};
use bevy_tasks::futures_lite::future;
use bevy_time::common_conditions::on_timer;
use tokio::sync::Mutex;
use crate::integration::accuweather::Controllers;

pub struct BirdNetPlugin;

impl Plugin for BirdNetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, birdnet_initial_retrieve)
            .add_systems(Startup, birdnet_process_retrieve)
            .add_systems(FixedUpdate, birdnet_retrieve.run_if(on_timer(birdnet_cadence())))
            .add_systems(FixedUpdate, birdnet_process_retrieve);

        app.world.spawn(BirdNetRecentDetections::new());
    }
}

fn birdnet_cadence() -> std::time::Duration {
    std::time::Duration::from_secs(10 * 60)
}

fn birdnet_render(mut query: Query<(&BirdList, &mut BirdNetRecentDetections)>) {
    for (renderer, detections) in query.iter_mut() {
        let names = detections
            .detections
            .iter()
            .map(|e| {
                let when = format!("{}:{}", e.timestamp.hour(), e.timestamp.minute());
                format!("• {} {}", when, e.species.common_name)
            })
            .collect::<Vec<_>>()
            .join("\n");
        renderer.text.text.render_text(names.as_str());
    }
}

fn birdnet_initial_retrieve(mut commands: Commands,
                            mut query: Query<(Entity, &Configuration, &BirdNetRecentDetections)>) {
    birdnet_retrieve(commands, query);
}

// FIXME: This might be possible to generify?
#[derive(Component)]
pub struct BirdNestTask {
    pub task: Task<Result<Response, reqwest::Error>>
}

fn birdnet_retrieve(mut commands: Commands,
                    mut query: Query<(Entity, &Configuration, &BirdNetRecentDetections)>) {
    let pool = AsyncComputeTaskPool::get();
    for (entity, configuration, recent) in query.iter() {
        println!("query birdnet");
        let task = pool.spawn(async move {
            let response = Client::new()
                .get(format!("{}/{}/detections",
                             BASE_URL,
                             configuration.token))
                .query(&[(
                    "from".to_string(),
                    recent.last_fetch
                        .map(|fetch| fetch.to_rfc3339())
                        .unwrap_or("".to_string()),
                )])
                .send().await;

            if let Ok(response) = response {
                ksdfjlfjklfjdsld
                need to process result here as the task will get executed and this will add componentswhich
                another system will get.
            }
        });

        commands.entity(entity).insert(BirdNestTask {
            task
        });
    }
}

fn birdnet_process_retrieve(mut commands: Commands,
                            mut query: Query<(&Configuration, &mut BirdNetRecentDetections, &mut BirdNestTask)>) {
    for (configuration, recent, task) in query.iter_mut() {
        if let Ok(response) = future::block_on(future::poll_once(&mut task.task)) {
            let data = response.json::<api::Envelope>();
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
                if let Some(backfill) = recent.detections.pop_front() {
                    detections.push(backfill);
                    num_short -= 1;
                } else {
                    break;
                }
            }

            recent.detections = detections.iter().cloned().collect();
        }
    }
}

const BASE_URL: &str = "https://app.birdweather.com/api/v1/stations";

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum BirdNetControllers {
    RecentDetections,
}

pub struct BirdNet {}

#[derive(Debug, Serialize, Deserialize, Component)]
pub struct Configuration {
    pub token: String,
    pub keep: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecentDetections {
    pub detections: Vec<api::Detection>,
}

#[derive(Default, Component)]
pub struct BirdNetRecentDetections {
    last_fetch: Option<DateTime<Utc>>,
    detections: VecDeque<api::Detection>,
}

impl BirdNetRecentDetections {
    pub fn new() -> Self {
        Self {
            last_fetch: None,
            detections: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct BirdList {
    text: FormattedText<RecentDetections, RecentDetections>,
}

impl BirdList {
    pub fn new(
        state: ModelKey<RecentDetections>,
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
