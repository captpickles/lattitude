use actix::dev::MessageResponse;
use actix::{Actor, Context, Handler, Message};
use chrono::Duration;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;
use toml::Value;

pub trait Controller: Send + 'static {
    type Configuration: Serialize + DeserializeOwned + Send;

    type Output: Send + PartialEq + 'static;

    fn identifier(&self) -> String;

    fn cadence(&self) -> Option<Duration>;

    fn configure(&mut self, configuration: Option<Self::Configuration>);

    fn update(&mut self) -> impl Future<Output = Option<Self::Output>> + Send;
}

pub struct ControllerManager<C, Configuration, Output>
where
    C: Controller<Configuration = Configuration, Output = Output>,
    Output: Send + 'static,
{
    identifier: String,
    cadence: Option<Duration>,
    controller: Arc<Mutex<C>>,
    state: Arc<Mutex<Option<C::Output>>>,
}

impl<C, Configuration, Output> ControllerManager<C, Configuration, Output>
where
    C: Controller<Configuration = Configuration, Output = Output>,
    Output: Send + 'static,
{
    pub fn new(controller: C) -> Self {
        Self {
            identifier: controller.identifier(),
            cadence: controller.cadence(),
            controller: Arc::new(Mutex::new(controller)),
            state: Arc::new(Mutex::new(None)),
        }
    }
}

impl<C, Configuration, Output> ControllerManager<C, Configuration, Output>
where
    C: Controller<Configuration = Configuration, Output = Output>,
    Output: Send + 'static,
{
    pub fn state(&self) -> Arc<Mutex<Option<Output>>> {
        self.state.clone()
    }
}

pub trait ManageableController {
    fn identifier(&self) -> String;

    fn cadence(&self) -> Option<Duration>;

    fn configure<'r>(
        &'r self,
        configuration: Option<toml::Value>,
    ) -> Pin<Box<dyn Future<Output = ()> + 'r>>;
    fn update<'r>(&'r self) -> Pin<Box<dyn Future<Output = ()> + 'r>>;
}

impl<C, Configuration, Output> ManageableController for ControllerManager<C, Configuration, Output>
where
    C: Controller<Configuration = Configuration, Output = Output>,
    Configuration: DeserializeOwned,
    Output: Send + PartialEq + 'static,
{
    fn identifier(&self) -> String {
        self.identifier.clone()
    }

    fn cadence(&self) -> Option<Duration> {
        self.cadence
    }

    fn configure<'r>(
        &'r self,
        configuration: Option<Value>,
    ) -> Pin<Box<dyn Future<Output = ()> + 'r>> {
        let controller = self.controller.clone();
        Box::pin(async move {
            let mut controller = controller.lock().await;
            if let Some(configuration) = configuration {
                if let Ok(config) = C::Configuration::deserialize(configuration) {
                    controller.configure(Some(config));
                } else {
                    controller.configure(None);
                }
            } else {
                controller.configure(None);
            }
        })
    }

    fn update<'r>(&'r self) -> Pin<Box<dyn Future<Output = ()> + 'r>> {
        let controller = self.controller.clone();
        let state = self.state.clone();

        Box::pin(async move {
            let v = controller.lock().await.update().await;
            *state.lock().await = v;
        })
    }
}

pub struct Controllers {
    controllers: Vec<Box<dyn ManageableController>>,
}

impl Controllers {
    pub fn new() -> Self {
        Self {
            controllers: vec![],
        }
    }

    pub async fn update(&self) {
        for controller in &self.controllers {
            controller.update().await;
        }
    }

    pub fn register<S>(&mut self, controller: impl Controller<Output = S>) -> Arc<Mutex<Option<S>>>
    where
        S: Send + Sync + PartialEq + 'static,
    {
        let managed = ControllerManager::new(controller);
        let state = managed.state();
        self.controllers.push(Box::new(managed));
        state
    }

    pub async fn configure(&self, identifier: &str, configuration: toml::Value) {
        if let Some(controller) = self
            .controllers
            .iter()
            .find(|e| e.identifier() == identifier)
        {
            controller.configure(Some(configuration)).await
        }
    }

    pub async fn unconfigure(&self, identifier: &str) {
        if let Some(controller) = self
            .controllers
            .iter()
            .find(|e| e.identifier() == identifier)
        {
            controller.configure(None).await
        }
    }
}
