pub mod api;

use engine::engine::integrations::IntegrationContext;
use engine::global_configuration::GlobalConfiguration;
use engine::integration::{Integration, IntegrationInfo};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::time::Duration;
use bevy_app::{App, FixedUpdate, Plugin, Startup};
use bevy_ecs::prelude::{Component, IntoSystemConfigs, Query};
use bevy_ecs::system::Res;
use bevy_time::common_conditions::on_timer;

pub struct AccuWeatherPlugin;

impl Plugin for AccuWeatherPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, hourly_initial_retrieve)
            .add_systems(Startup, daily_initial_retrieve)
            .add_systems(FixedUpdate, hourly_retrieve.run_if(on_timer(hourly_cadence())))
            .add_systems(FixedUpdate, daily_retrieve.run_if(on_timer(daily_cadence())));

        app.world.spawn(Controllers::Hourly);
    }
}

fn hourly_cadence() -> Duration {
    Duration::from_secs(10 * 60)
}

fn hourly_initial_retrieve() {
}

fn hourly_retrieve() {
}

fn daily_cadence() -> Duration {
    Duration::from_secs(10 * 60)
}

fn daily_initial_retrieve() {
}

fn daily_retrieve() {
}

#[derive(Clone, Debug, Serialize, Deserialize, Component)]
pub struct Configuration {
    api_key: String,
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone, Component)]
pub enum Controllers {
    Daily,
    Hourly,
}
