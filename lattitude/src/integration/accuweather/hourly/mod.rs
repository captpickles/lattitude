pub mod api;

use crate::integration::accuweather::daily::api::DailyForecast;
use crate::integration::accuweather::hourly::api::HourlyForecast;
use crate::integration::accuweather::AccuWeather;
use chrono::Duration;
use engine::model::Model;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Hourly {
    model: Model<Vec<HourlyForecast>>,
}

impl Hourly {
    pub fn new() -> Self {
        Self {
            model: Model::default(),
        }
    }

    pub fn model(&self) -> Model<Vec<HourlyForecast>> {
        self.model.clone()
    }
}

impl Hourly {
    fn update(&mut self) -> impl Future<Output = ()> + Send + Sync {
        async move { todo!() }
    }
}
