use actix::dev::MessageResponse;
use actix::{Actor, Context, Handler, Message};
use chrono::{DateTime, Duration, Utc};
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

    fn cadence(&self) -> Duration;

    fn configure(&mut self, configuration: Option<Self::Configuration>);

    fn update(&mut self) -> impl Future<Output = Option<Self::Output>> + Send + Sync;
}

pub struct ControllerManager<C, Configuration, Output>
where
    C: Controller<Configuration = Configuration, Output = Output>,
    Output: Send + 'static,
{
    identifier: String,
    cadence: Duration,
    controller: Arc<Mutex<C>>,
    state: Arc<Mutex<Option<C::Output>>>,
    last_update: Arc<Mutex<Option<DateTime<Utc>>>>,
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
            last_update: Arc::new(Mutex::new(None)),
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

pub trait ManageableController: Sync + Send {
    fn identifier(&self) -> String;

    fn cadence(&self) -> Duration;

    fn configure<'r>(
        &'r self,
        configuration: Option<toml::Value>,
    ) -> Pin<Box<dyn Future<Output = ()> + 'r>>;
    fn update<'r>(&'r self) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'r>>;
}

impl<C, Configuration, Output> ManageableController for ControllerManager<C, Configuration, Output>
where
    C: Controller<Configuration = Configuration, Output = Output> + Sync,
    Configuration: DeserializeOwned,
    Output: Send + PartialEq + Sync + 'static,
{
    fn identifier(&self) -> String {
        self.identifier.clone()
    }

    fn cadence(&self) -> Duration {
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

    fn update<'r>(&'r self) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'r>> {
        let controller = self.controller.clone();
        let state = self.state.clone();
        let last_update = self.last_update.clone();

        Box::pin(async move {
            let last_update_time = self.last_update.lock().await.clone();

            let should_update = if let Some(last_update) = last_update_time {
                if (Utc::now() - last_update) > self.cadence {
                    true
                } else {
                    false
                }
            } else {
                true
            };

            if should_update {
                let v = controller.lock().await.update().await;
                *state.lock().await = v;
                *last_update.lock().await = Some(Utc::now());
            }
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

    pub fn register<S>(&mut self, controller: impl Controller<Output = S> + Sync) -> Arc<Mutex<Option<S>>>
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

    pub async fn run(&self) {
        loop {
            self.update().await;
            tokio::time::sleep(Duration::seconds(5).to_std().unwrap()).await;
        }
    }
}
