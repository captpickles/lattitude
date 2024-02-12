use actix::dev::MessageResponse;
use actix::{Actor, Context, Handler, Message};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::{Arc};
use tokio::sync::Mutex;
use tokio::task::spawn_blocking;
use toml::Value;

pub trait Controller: Send + 'static {
    type Configuration: Serialize + DeserializeOwned + Send;

    type Output: Send + PartialEq + 'static;

    fn configure(&mut self, configuration: Option<Self::Configuration>);

    fn update(&mut self) -> impl Future<Output = Option<Self::Output>> + Send;
}

pub struct ControllerManager<C, Configuration, Output>
where
    C: Controller<Configuration = Configuration, Output = Output>,
    Output: Send + 'static,
{
    controller: Arc<Mutex<C>>,
    state: Arc<Mutex<Option<C::Output>>>,
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
    fn configure(&self, configuration: toml::Value);
    fn update(&self);
}

impl<C, Configuration, Output> ManageableController for ControllerManager<C, Configuration, Output>
where
    C: Controller<Configuration = Configuration, Output = Output>,
    Configuration: DeserializeOwned,
    Output: Send + PartialEq + 'static,
{
    fn configure(&self, configuration: Value) {
        let controller = self.controller.clone();
        tokio::spawn(async move {
            let mut controller = controller.lock().await;
            if let Ok(config) = C::Configuration::deserialize(configuration) {
                controller.configure(Some(config));
            } else {
                controller.configure(None);
            }
        });
    }

    fn update(&self) {
        let controller = self.controller.clone();
        let state = self.state.clone();

        tokio::spawn(async move {
            let new_state = controller.lock().await.update().await;
            let mut cur_state = state.lock().await;
            match (&mut *cur_state, new_state) {
                (Some(ref mut cur), Some(new)) => {
                    *cur = new
                },
                (None, Some(new)) => {
                    cur_state.replace(new);
                }
                _ => {

                }

            }
            /*
            if let Some(cur_state) = &mut *cur_state {
                if new_state != *cur_state {
                    *cur_state = new_state;
                }
            } else {
                cur_state.replace(new_state);
            }
             */
        });
    }
}

pub struct Controllers {
    controllers: Vec<Box<dyn ManageableController>>,
}
