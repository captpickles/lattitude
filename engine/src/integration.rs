use std::future::Future;
use std::hash::Hash;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::engine::integrations::IntegrationContext;
use crate::global_configuration::GlobalConfiguration;

pub struct IntegrationInfo {
    pub key: String,
    pub name: String,
}

impl IntegrationInfo {
    pub fn new(key: &str, name: &str) -> Self {
        Self {
            key: key.to_string(),
            name: name.to_string(),
        }
    }
}

pub trait Integration: Send + Sync + 'static {
    type Discriminant: Copy + Hash + PartialEq + Eq + Send + Sync;

    type Configuration: Clone + Serialize + DeserializeOwned + Send + Sync;

    fn info() -> IntegrationInfo;

    fn integrate(&self, context: &mut IntegrationContext<Self>)
    where
        Self: Sized;

    fn configure(
        &mut self,
        global_configuration: GlobalConfiguration,
        integration_configuration: Option<Self::Configuration>,
    ) -> impl Future<Output = ()> + Send + Sync;

    fn update(
        &mut self,
        discriminant: Self::Discriminant,
    ) -> impl Future<Output = ()> + Send + Sync;
}
