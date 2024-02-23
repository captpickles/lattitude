use crate::global_configuration::GlobalConfiguration;
use crate::integration::Integration;
use crate::model::{Model, ModelManager};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use toml::Value;

pub trait ManageableIntegration {
    fn configure<'r>(
        &'r self,
        world: &'r GlobalConfiguration,
        configuration: Option<toml::Value>,
    ) -> Pin<Box<dyn Future<Output = ()> + 'r>>;

    fn update<'r>(&'r self) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'r>>;
}

pub struct IntegrationHolder<I>
where
    I: Integration,
{
    integration: Arc<Mutex<I>>,
    updates: Arc<Mutex<HashMap<I::Discriminant, UpdateEntry>>>,
}

impl<I> IntegrationHolder<I>
where
    I: Integration,
{
    pub fn new(integration: I, cadences: HashMap<I::Discriminant, Duration>) -> Self {
        Self {
            integration: Arc::new(Mutex::new(integration)),
            updates: Arc::new(Mutex::new(
                cadences
                    .iter()
                    .map(|(discriminant, cadence)| (*discriminant, UpdateEntry::new(*cadence)))
                    .collect(),
            )),
        }
    }
}

pub struct UpdateEntry {
    cadence: Duration,
    last_update: Option<DateTime<Utc>>,
}

impl UpdateEntry {
    fn new(cadence: Duration) -> Self {
        Self {
            cadence,
            last_update: None,
        }
    }

    fn should_update(&self) -> bool {
        if let Some(last_update) = self.last_update {
            (Utc::now() - last_update) > self.cadence
        } else {
            true
        }
    }

    fn mark_updated(&mut self) {
        self.last_update.replace(Utc::now());
    }
}

impl<I> ManageableIntegration for IntegrationHolder<I>
where
    I: Integration,
{
    fn configure<'r>(
        &'r self,
        global_configuration: &'r GlobalConfiguration,
        configuration: Option<Value>,
    ) -> Pin<Box<dyn Future<Output = ()> + 'r>> {
        let controller = self.integration.clone();
        Box::pin(async move {
            let mut integration = controller.lock().await;
            if let Some(configuration) = configuration {
                if let Ok(config) = <I as Integration>::Configuration::deserialize(configuration) {
                    integration
                        .configure(global_configuration.clone(), Some(config))
                        .await;
                } else {
                    integration
                        .configure(global_configuration.clone(), None)
                        .await;
                }
            } else {
                integration
                    .configure(global_configuration.clone(), None)
                    .await;
            }
        })
    }

    fn update<'r>(&'r self) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'r>> {
        Box::pin(async move {
            let mut updates = self.updates.lock().await;

            let mut integration = self.integration.lock().await;

            for (discriminant, entry) in updates.iter_mut() {
                if entry.should_update() {
                    integration.update(*discriminant).await;
                    entry.mark_updated();
                }
            }
        })
    }
}

struct IntegrationEntry {
    managed: Box<dyn ManageableIntegration>,
}

#[derive(Default)]
pub struct Integrations {
    integrations: Vec<IntegrationEntry>,
}

impl Integrations {
    pub fn register<I>(&mut self, state_manager: &mut ModelManager, integration: I)
    where
        I: Integration,
    {
        let mut ctx = IntegrationContext::new(state_manager);
        integration.integrate(&mut ctx);
        let managed = IntegrationHolder::new(integration, ctx.cadences);
        self.integrations.push(IntegrationEntry {
            managed: Box::new(managed),
        });
    }
}

pub struct IntegrationContext<'ctx, I>
where
    I: Integration,
{
    model_manager: &'ctx mut ModelManager,
    cadences: HashMap<I::Discriminant, Duration>,
}

impl<'ctx, I: Integration> IntegrationContext<'ctx, I> {
    fn new(state_manager: &'ctx mut ModelManager) -> Self {
        Self {
            model_manager: state_manager,
            cadences: Default::default(),
        }
    }
    pub fn register_controller(&mut self, discriminant: I::Discriminant, cadence: Duration) {
        self.cadences.insert(discriminant, cadence);
    }

    pub fn register_model<T>(&mut self, state: Model<T>) -> ModelRegistration<T>
    where
        T: Clone + Debug + Sync + Send + 'static,
    {
        self.model_manager.register(TypeId::of::<I>(), state);
        ModelRegistration {
            model_manager: self.model_manager,
            _marker: Default::default(),
        }
    }
}

pub struct ModelRegistration<'ctx, M> {
    model_manager: &'ctx mut ModelManager,
    _marker: PhantomData<M>,
}

impl<M> ModelRegistration<'_, M>
where
    M: Debug + Clone + Sync + Send + 'static,
{
    pub fn provides<Output>(self) -> Self
    where
        Output: Debug + From<M> + 'static,
    {
        self.model_manager.provides::<M, Output>();
        self
    }
}
