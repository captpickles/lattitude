pub mod api;

use crate::integration::accuweather::daily::api::DailyForecast;
use crate::integration::accuweather::{AccuWeather, Configuration};
use chrono::Duration;
use engine::model::Model;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Daily {
    model: Model<Vec<DailyForecast>>,
}

impl Daily {
    pub fn new() -> Self {
        Self {
            model: Model::default(),
        }
    }

    pub fn model(&self) -> Model<Vec<DailyForecast>> {
        self.model.clone()
    }
}

impl Daily {
    pub fn update(&mut self, config: &Configuration) -> impl Future<Output = ()> + Send + Sync {
        async move { todo!() }
    }
}
