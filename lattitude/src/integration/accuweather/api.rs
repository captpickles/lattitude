use serde::Deserialize;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Wind {
    pub speed: WindSpeed,
    pub direction: WindDirection,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WindSpeed {
    pub value: f32,
}

impl Hash for WindSpeed {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.value.to_be_bytes())
    }
}

#[derive(Debug, Clone, Deserialize, Hash, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WindDirection {
    pub degrees: u16,
    pub localized: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TotalLiquid {
    pub value: f32,
}

impl Hash for TotalLiquid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.value.to_be_bytes())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Snow {
    pub value: f32,
}

impl Hash for Snow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.value.to_be_bytes())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Rain {
    pub value: f32,
}

impl Hash for Rain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.value.to_be_bytes())
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Ice {
    pub value: f32,
}

impl Hash for Ice {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.value.to_be_bytes())
    }
}
