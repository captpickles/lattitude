pub mod integrations;

use crate::engine::integrations::Integrations;
use crate::integration::Integration;
use crate::model::ModelManager;

#[derive(Default)]
pub struct Engine {
    state_manager: ModelManager,
    integrations: Integrations,
}

impl Engine {
    pub fn register<I: Integration>(&mut self, integration: I) {
        self.integrations
            .register(&mut self.state_manager, integration);
    }

    pub async fn run(&self) {}
}

#[cfg(test)]
mod test {
    use std::future::Future;

    use chrono::Duration;

    use crate::engine::integrations::IntegrationContext;
    use crate::engine::Engine;
    use crate::integration::{Integration, IntegrationInfo};

    struct AccuWeather {
        hourly: HourlyForecast,
    }

    struct HourlyForecast {}

    #[derive(Hash, Eq, PartialEq, Copy, Clone)]
    enum AccuWeatherControllers {
        Hourly,
        Daily,
    }

    impl Integration for AccuWeather {
        type Discriminant = AccuWeatherControllers;
        type Configuration = ();

        fn info() -> IntegrationInfo {
            IntegrationInfo::new("AccuWeather".to_string())
        }

        fn integrate(&self, context: &mut IntegrationContext<Self>)
        where
            Self: Sized,
        {
            context.register_controller(AccuWeatherControllers::Hourly, Duration::minutes(5));
            context.register_controller(AccuWeatherControllers::Daily, Duration::minutes(30));
        }

        fn configure(&mut self, controller_config: Option<Self::Configuration>) {
            // nothing
        }

        fn update(
            &mut self,
            discriminant: Self::Discriminant,
        ) -> impl Future<Output = ()> + Send + Sync {
            async move {
                match discriminant {
                    AccuWeatherControllers::Hourly => {}
                    AccuWeatherControllers::Daily => {}
                }
            }
        }
    }

    #[tokio::test]
    async fn whut() {
        let mut engine = Engine::new();
        engine.register(AccuWeather {
            hourly: HourlyForecast {},
        });

        engine.run().await;
    }
}
