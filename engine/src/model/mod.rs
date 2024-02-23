use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Default, Clone)]
pub struct Model<T>
where
    T: Clone + Sync + Send + Debug + 'static,
{
    inner: Arc<Mutex<Option<T>>>,
}

impl<T> Model<T>
where
    T: Clone + Sync + Send + Debug + 'static,
{
    pub async fn update(&self, value: T) {
        self.inner.lock().await.replace(value);
    }

    pub async fn clear(&self) {
        self.inner.lock().await.take();
    }

    pub async fn get(&self) -> Option<T> {
        self.inner.lock().await.clone()
    }
}

#[derive(Clone)]
pub struct ModelKey<T>
where
    T: Clone + Debug + 'static,
{
    provider: TypeId,
    _marker: PhantomData<T>,
}

impl<T> Debug for ModelKey<T>
where
    T: Clone + Debug + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.provider)
    }
}

#[derive(Default)]
pub struct ModelManager {
    primary: HashMap<TypeId, ProviderEntry>,
    convertable: HashMap<TypeId, Vec<ConverterEntry>>,
}

impl ModelManager {
    pub fn register<T>(&mut self, provider: TypeId, state: Model<T>)
    where
        T: Clone + Sync + Send + Debug + 'static,
    {
        let key = TypeId::of::<T>();
        let entry = ProviderEntry {
            provider,
            state: Box::new(state),
        };
        self.primary.insert(key, entry);
    }

    pub fn provides<Input, Output>(&mut self)
    where
        Input: Debug + Clone + Sync + Send + 'static,
        Output: Debug + From<Input> + 'static,
    {
        let converter = FromConverter::<Input, Output>::new();

        let output_key = TypeId::of::<Output>();
        let input_key = TypeId::of::<Input>();

        let entries = self.convertable.entry(output_key).or_default();
        entries.push(ConverterEntry {
            input_key,
            converter: Box::new(converter),
        });
    }

    pub fn providers_for<T>(&self) -> Vec<ModelKey<T>>
    where
        T: Debug + Clone + 'static,
    {
        let mut keys = Vec::new();
        if let Some(entries) = self.convertable.get(&TypeId::of::<T>()) {
            for each in entries {
                if let Some(primary) = self.primary.get(&each.input_key) {
                    keys.push(ModelKey {
                        provider: primary.provider,
                        _marker: Default::default(),
                    })
                }
            }
        }

        keys
    }

    pub async fn get_all<T>(&self) -> Vec<Option<T>>
    where
        T: Debug + Clone + Sync + Send + 'static,
    {
        let key = TypeId::of::<T>();

        if let Some(primary) = self.primary.get(&key) {
            if let Some(value) = primary.state.downcast_ref::<Model<T>>() {
                return vec![value.get().await];
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
                        if let Some(output) = output.downcast_ref::<Option<T>>() {
                            values.push(output.clone())
                        }
                    }
                }
            }
        }

        values
    }

    pub async fn get<T>(&self, key: &ModelKey<T>) -> Option<T>
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
    provider: TypeId,
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
    In: Debug + Clone + Sync + Send + 'static,
    Out: Debug + From<In> + 'static,
{
    fn convert<'i>(
        &'i self,
        input: &'i Box<dyn Any>,
    ) -> Pin<Box<dyn Future<Output = Option<Box<dyn Any>>> + '_>> {
        Box::pin(async move {
            if let Some(input_value) = input.downcast_ref::<Model<In>>() {
                let input_value = input_value.get().await;
                let output_value: Option<Out> = input_value.clone().map(|inner| inner.into());
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

    /*
    #[tokio::test]
    async fn whut() {
        let mut manager = ModelManager::new();

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
            println!("{:?}", manager.get(&each).await);
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

     */
}
