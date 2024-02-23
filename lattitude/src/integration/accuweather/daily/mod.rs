pub mod api;

use std::future::Future;
use std::sync::Arc;
use chrono::Duration;
use tokio::sync::Mutex;
use crate::integration::accuweather::AccuWeather;
use crate::integration::accuweather::daily::api::DailyForecast;

pub struct Daily {
    model: Arc<Mutex<Option<Vec<DailyForecast>>>>
}

impl Daily {
    pub fn new() -> Self {
        Self {
            model: Arc::new(Mutex::new( None ) )
        }
    }

    pub fn model(&self) -> Arc<Mutex<Option<Vec<DailyForecast>>>> {
        self.model.clone()
    }

}

impl Daily {


    fn update(&mut self) -> impl Future<Output=()> + Send + Sync {
        async move {
            todo!()
        }
    }
}
