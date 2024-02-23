mod hourly;
mod daily;
pub mod api;

use std::future::Future;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use engine::engine::integrations::IntegrationContext;
use engine::integration::{Integration, IntegrationInfo};
use crate::integration::accuweather::daily::api::DailyForecast;
use crate::integration::accuweather::daily::Daily;
use crate::integration::accuweather::hourly::api::HourlyForecast;
use crate::integration::accuweather::hourly::Hourly;


pub struct AccuWeather {
    hourly: Hourly,
    daily: Daily,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Configuration {
    api_key: String,
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Controllers {
    Daily,
    Hourly,
}

impl Integration for AccuWeather {
    type Discriminant = Controllers;
    type Configuration = Configuration;

    fn info() -> IntegrationInfo {
        IntegrationInfo {
            name: "AccuWeather".to_string(),
        }
    }

    fn integrate(&self, context: &mut IntegrationContext<Self>) where Self: Sized {
        context.register_controller( Controllers::Daily, Duration::minutes(60));
        context.register_controller( Controllers::Hourly, Duration::minutes(10));

        context.register_model::<Vec<DailyForecast>>(
            self.daily.model()
        );

        context.register_model::<Vec<HourlyForecast>>(
            self.hourly.model()
        );
    }

    fn configure(&mut self, controller_config: Option<Self::Configuration>) {
        todo!()
    }

    fn update(&mut self, discriminant: Self::Discriminant) -> impl Future<Output=()> + Send + Sync {
        async move {
            todo!()
        }
    }
    //type Controllers = Controllers;

    /*
    fn create_controller(&self, controller: Self::Controllers) -> impl Controller {
        match controller {
            Controllers::Daily => {
                Daily::new()
            }
            Controllers::Hourly => {
                todo!()
                //Hourly::new()
            }
        }
    }

     */
}