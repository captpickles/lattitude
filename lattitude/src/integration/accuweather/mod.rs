pub mod api;
mod daily;
mod hourly;

use crate::integration::accuweather::daily::api::DailyForecast;
use crate::integration::accuweather::daily::Daily;
use crate::integration::accuweather::hourly::api::HourlyForecast;
use crate::integration::accuweather::hourly::Hourly;
use chrono::Duration;
use engine::engine::integrations::IntegrationContext;
use engine::global_configuration::GlobalConfiguration;
use engine::integration::{Integration, IntegrationInfo};
use serde::{Deserialize, Serialize};
use std::future::Future;

pub struct AccuWeather {
    global_configuration: GlobalConfiguration,
    configuration: Option<Configuration>,
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
        IntegrationInfo::new("accuweather", "AccuWeather")
    }

    fn integrate(&self, context: &mut IntegrationContext<Self>)
    where
        Self: Sized,
    {
        context.register_controller(Controllers::Hourly, Duration::minutes(10));
        context.register_model::<Vec<HourlyForecast>>(self.hourly.model());

        context.register_controller(Controllers::Daily, Duration::minutes(60));
        context.register_model::<Vec<DailyForecast>>(self.daily.model());
    }

    async fn configure(
        &mut self,
        global_configuration: GlobalConfiguration,
        integration_configuration: Option<Self::Configuration>,
    ) {
        self.global_configuration = global_configuration;
        self.configuration = integration_configuration;
    }

    async fn update(&mut self, discriminant: Self::Discriminant) {
        if let Some(config) = &self.configuration {
            match discriminant {
                Controllers::Daily => {
                    self.daily.update(config);
                }
                Controllers::Hourly => {
                    self.daily.update(config);
                }
            }
        }
    }
}
