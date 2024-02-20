use crate::controller::{Controller, ControllerManager, Controllers};
use chrono::Duration;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use toml::Value;

pub trait Integration: Send + 'static {
    type Configuration: Serialize + DeserializeOwned + Send;

    type Controllers;

    fn create_controller(&self, controller: Self::Controllers) -> impl Controller;
}

pub trait ManageableIntegration {
    fn identifier(&self) -> String;

    fn configure<'r>(
        &'r self,
        configuration: Option<toml::Value>,
    ) -> Pin<Box<dyn Future<Output=()> + 'r>>;

    fn update<'r>(&'r self) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'r>>;
}

pub struct IntegrationManager<I, Configuration>
    where
        I: Integration<Configuration=Configuration>,
{
    identifier: String,
    integration: Arc<Mutex<I>>,
}

impl<I, Configuration> IntegrationManager<I, Configuration> where
    I: Integration<Configuration=Configuration>
{
    pub fn new(integration: I) -> Self {
        Self {
            identifier: "".to_string(),
            integration: Arc::new(Mutex::new(integration)),
        }
    }
}

impl<I, Configuration> ManageableIntegration for IntegrationManager<I, Configuration> where
    I: Integration<Configuration=Configuration> {
    fn identifier(&self) -> String {
        todo!()
    }

    fn configure<'r>(&'r self, configuration: Option<Value>) -> Pin<Box<dyn Future<Output=()> + 'r>> {
        todo!()
    }

    fn update<'r>(&'r self) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'r>> {
        todo!()
    }
}

pub struct Integrations {
    integrations: Vec<Box<dyn ManageableIntegration>>,
}

impl Integrations {
    pub fn register(&mut self, integration: impl Integration + Sync) {
        let managed = IntegrationManager::new(integration);
        self.integrations.push(Box::new(managed));
        //let managed = ControllerManager::new(controller);
        //let state = managed.state();
        //self.controllers.push(Box::new(managed));
        //state
    }
}
