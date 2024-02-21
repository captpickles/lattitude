use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

static PROVIDER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct Provider {
    id: usize,
    name: String,
}

impl PartialEq for Provider {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Provider {
    pub fn new(name: &str) -> Self {
        Self {
            id: PROVIDER_COUNTER.fetch_add(1, Ordering::Relaxed),
            name: name.to_string(),
        }
    }
}

pub struct DataKey<T>
where
    T: Clone + Debug + 'static,
{
    provider: Provider,
    _marker: PhantomData<T>,
}

impl<T> Debug for DataKey<T>
where
    T: Clone + Debug + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.provider)
    }
}

pub struct StateManager {
    primary: HashMap<TypeId, ProviderEntry>,
    convertable: HashMap<TypeId, Vec<ConverterEntry>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            primary: Default::default(),
            convertable: Default::default(),
        }
    }

    pub fn register<T>(&mut self, provider: &Provider, state: Arc<Mutex<T>>)
    where
        T: 'static,
    {
        let key = TypeId::of::<T>();
        let entry = ProviderEntry {
            provider: provider.clone(),
            state: Box::new(state),
        };
        self.primary.insert(key, entry);
    }

    pub fn provides<Input, Output>(&mut self)
    where
        Input: Debug + Clone + 'static,
        Output: Debug + From<Input> + 'static,
    {
        let converter = FromConverter::<Input, Output>::new();

        let output_key = TypeId::of::<Output>();
        let input_key = TypeId::of::<Input>();

        let mut entries = self.convertable.entry(output_key).or_default();
        entries.push(ConverterEntry {
            input_key,
            converter: Box::new(converter),
        });
    }

    pub fn providers_for<T>(&self) -> Vec<DataKey<T>>
    where
        T: Debug + Clone + 'static,
    {
        let mut keys = Vec::new();
        if let Some(entries) = self.convertable.get(&TypeId::of::<T>()) {
            for each in entries {
                if let Some(primary) = self.primary.get(&each.input_key) {
                    keys.push(DataKey {
                        provider: primary.provider.clone(),
                        _marker: Default::default(),
                    })
                }
            }
        }

        keys
    }

    pub async fn get_all<T>(&self) -> Vec<T>
    where
        T: Debug + Clone + 'static,
    {
        let key = TypeId::of::<T>();

        if let Some(primary) = self.primary.get(&key) {
            if let Some(value) = primary.state.downcast_ref::<Arc<Mutex<T>>>() {
                return vec![value.lock().await.clone()];
            }
        }

        let mut values = vec![];
        if let Some(entries) = self.convertable.get(&key) {
            for ConverterEntry {
                input_key,
                converter,
            } in entries
            {
                if let Some(primary) = self.primary.get(input_key) {
                    let output = converter.convert(&primary.state).await;
                    if let Some(output) = output {
                        if let Some(output) = output.downcast_ref::<T>() {
                            values.push(output.clone())
                        }
                    }
                }
            }
        }

        values
    }

    pub async fn get<T>(&self, key: DataKey<T>) -> Option<T>
    where
        T: Clone + Debug + 'static,
    {
        if let Some(entries) = self.convertable.get(&TypeId::of::<T>()) {
            if let Some(provider_entry) = entries.iter().find(|inner| {
                if let Some(primary) = self.primary.get(&inner.input_key) {
                    if primary.provider == key.provider {
                        return true;
                    }
                }
                false
            }) {
                if let Some(primary) = self.primary.get(&provider_entry.input_key) {
                    let output = provider_entry.converter.convert(&primary.state).await;
                    if let Some(output) = output {
                        if let Some(output) = output.downcast_ref::<T>() {
                            return Some(output.clone());
                        }
                    }
                }
            }
        }

        None
    }
}

struct ProviderEntry {
    provider: Provider,
    state: Box<dyn Any>,
}

struct ConverterEntry {
    input_key: TypeId,
    converter: Box<dyn Converter>,
}

trait Converter {
    fn convert<'i>(
        &'i self,
        input: &'i Box<dyn Any>,
    ) -> Pin<Box<dyn Future<Output = Option<Box<dyn Any>>> + '_>>;
}

struct FromConverter<In, Out> {
    _marker: PhantomData<(In, Out)>,
}

impl<In, Out> FromConverter<In, Out> {
    pub fn new() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<In, Out> Converter for FromConverter<In, Out>
where
    In: Debug + Clone + 'static,
    Out: Debug + From<In> + 'static,
{
    fn convert<'i>(
        &'i self,
        input: &'i Box<dyn Any>,
    ) -> Pin<Box<dyn Future<Output = Option<Box<dyn Any>>> + '_>> {
        Box::pin(async move {
            if let Some(input_value) = input.downcast_ref::<Arc<Mutex<In>>>() {
                let input_value = input_value.lock().await;
                let output_value: Out = input_value.clone().into();
                let boxed: Box<dyn Any> = Box::new(output_value);
                return Some(boxed);
            }

            None
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[derive(Clone, Debug)]
    pub struct AccuWeather {
        wind_direction: u32,
        wind_speed: u32,
    }

    impl From<AccuWeather> for WindDirection {
        fn from(value: AccuWeather) -> Self {
            Self {
                dir: value.wind_direction,
            }
        }
    }

    impl From<AccuWeather> for WindSpeed {
        fn from(value: AccuWeather) -> Self {
            Self {
                speed: value.wind_speed,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct WeatherChannel {
        direction_of_the_wind: u32,
        speed_of_the_wind: u32,
    }

    impl From<WeatherChannel> for WindDirection {
        fn from(value: WeatherChannel) -> Self {
            Self {
                dir: value.direction_of_the_wind,
            }
        }
    }

    impl From<WeatherChannel> for WindSpeed {
        fn from(value: WeatherChannel) -> Self {
            Self {
                speed: value.speed_of_the_wind,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct WindDirection {
        dir: u32,
    }

    #[derive(Clone, Debug)]
    pub struct WindSpeed {
        speed: u32,
    }

    #[derive(Clone, Debug)]
    pub struct BirdNet {}

    #[tokio::test]
    async fn whut() {
        let mut manager = StateManager::new();

        let accuweather_provider = Provider::new("AccuWeather");
        let weather_channel_provider = Provider::new("The Weather Channel");
        let birdnet_provider = Provider::new("BirdNET");

        let accuweather = Arc::new(Mutex::new(AccuWeather {
            wind_direction: 180,
            wind_speed: 200,
        }));
        let weather_channel = Arc::new(Mutex::new(WeatherChannel {
            direction_of_the_wind: 188,
            speed_of_the_wind: 110,
        }));
        let birdnet = Arc::new(Mutex::new(BirdNet {}));

        manager.register(&accuweather_provider, accuweather.clone());
        manager.register(&weather_channel_provider, weather_channel.clone());
        manager.register(&birdnet_provider, birdnet.clone());

        manager.provides::<AccuWeather, WindDirection>();
        manager.provides::<WeatherChannel, WindDirection>();
        manager.provides::<AccuWeather, WindSpeed>();
        manager.provides::<WeatherChannel, WindSpeed>();

        // not allowed
        //manager.alias::<BirdNet, WindDirection>();

        let keys = manager.providers_for::<WindSpeed>();
        println!("{:#?}", keys);

        for each in keys {
            println!("{:?}", manager.get(each).await);
        }

        let values = manager.get_all::<WindDirection>().await;
        println!("{:#?}", values);

        accuweather.lock().await.wind_direction = 99;
        accuweather.lock().await.wind_speed = 42;

        let values = manager.get_all::<WindDirection>().await;
        println!("{:#?}", values);

        let values = manager.get_all::<WindSpeed>().await;
        println!("{:#?}", values);

        let values = manager.get_all::<BirdNet>().await;
        println!("{:#?}", values);
    }
}
