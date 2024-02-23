mod directory;

use toml::Value;

pub struct Configuration {
    key: String,
    content: Value,
}

pub trait ConfigurationLoader {
    async fn load(&mut self) -> Vec<Configuration>;
}
