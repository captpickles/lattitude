use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct StateManager {
    primary: HashMap<TypeId, Box<dyn Any>>,
    convertable: HashMap<TypeId, Vec<ConverterEntry>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            primary: Default::default(),
            convertable: Default::default(),
        }
    }

    pub fn register<T>(&mut self, state: Arc<Mutex<T>>)
        where
            T: 'static,
    {
        let key = TypeId::of::<T>();
        self.primary.insert(key, Box::new(state));
    }

    pub fn alias<Input, Output>(&mut self)
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

    pub async fn get<T>(&self) -> Vec<T>
        where
            T: Debug + Clone + 'static,
    {
        let key = TypeId::of::<T>();

        if let Some(primary) = self.primary.get(&key) {
            if let Some(value) = primary.downcast_ref::<Arc<Mutex<T>>>() {
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
                    let output = converter.convert(primary).await;
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

    impl From<AccuWeather> for WindSpeed{
        fn from(value: AccuWeather) -> Self {
            Self {
                speed: value.wind_speed
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
                speed: value.speed_of_the_wind
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

        let accuweather = Arc::new(Mutex::new(AccuWeather {
            wind_direction: 180,
            wind_speed: 200,
        }));
        let weather_channel = Arc::new(Mutex::new(WeatherChannel {
            direction_of_the_wind: 188,
            speed_of_the_wind: 110,
        }));
        let birdnet = Arc::new(Mutex::new(BirdNet {}));

        manager.register(accuweather.clone());
        manager.register(weather_channel.clone());
        manager.register(birdnet.clone());

        manager.alias::<AccuWeather, WindDirection>();
        manager.alias::<WeatherChannel, WindDirection>();
        manager.alias::<AccuWeather, WindSpeed>();
        manager.alias::<WeatherChannel, WindSpeed>();

        // not allowed
        //manager.alias::<BirdNet, WindDirection>();

        let values = manager.get::<WindDirection>().await;
        println!("{:#?}", values);

        accuweather.lock().await.wind_direction = 99;
        accuweather.lock().await.wind_speed = 42;

        let values = manager.get::<WindDirection>().await;
        println!("{:#?}", values);

        let values = manager.get::<WindSpeed>().await;
        println!("{:#?}", values);

        let values = manager.get::<BirdNet>().await;
        println!("{:#?}", values);
    }
}
