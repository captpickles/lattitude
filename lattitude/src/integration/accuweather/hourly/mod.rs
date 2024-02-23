pub mod api;

use std::future::Future;
use std::sync::{Arc};
use chrono::Duration;
use tokio::sync::Mutex;
use crate::integration::accuweather::AccuWeather;
use crate::integration::accuweather::daily::api::DailyForecast;
use crate::integration::accuweather::hourly::api::HourlyForecast;

pub struct Hourly {
    model: Arc<Mutex<Option<Vec<HourlyForecast>>>>
}

impl Hourly {
    pub fn new() -> Self {
        Self {
            model: Arc::new(Mutex::new(None ) ),
        }
    }


    pub fn model(&self) -> Arc<Mutex<Option<Vec<HourlyForecast>>>> {
        self.model.clone()
    }

}

impl Hourly {
    fn update(&mut self) -> impl Future<Output=()> + Send + Sync {
        async move {
            todo!()
        }
    }
}