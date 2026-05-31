use std::{convert::Infallible, str::FromStr, string::ParseError};

use thiserror::Error;

pub struct Nutritions {
    energy: NutritionEnergy,
    fat: NutritionValue,
    carbohydrates: NutritionValue,
    salt: NutritionValue,
    proteins: NutritionValue,

    extra_values: Vec<NutritionValue>,
}

pub enum NutritionValueType {
    Gram(f64),
    Kilogram(f64),
    Percent(f64),
}

impl NutritionValueType {
    pub fn get_options() -> Vec<(String, String)> {
        [("g", "gram"), ("%", "percent"), ("kg", "kilogram")]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum ValueParseError {
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),
}
impl TryFrom<(&str, &str)> for NutritionValueType {
    type Error = ValueParseError;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let (key, value) = value;
        let value: f64 = value
            .parse()
            .map_err(|e: std::num::ParseFloatError| ValueParseError::InvalidValue(e.to_string()))?;
        match key {
            "g" => Ok(Self::Gram(value)),
            "kg" => Ok(Self::Kilogram(value)),
            "%" => Ok(Self::Percent(value / 100.0)),
            _ => Err(ValueParseError::InvalidKey(key.to_string())),
        }
    }
}

pub struct NutritionEnergy {
    kcal: f64,
}

impl NutritionEnergy {
    pub fn from_kj(value: f64) -> Self {
        Self {
            kcal: value / 4.184,
        }
    }

    pub fn as_kcal(value: f64) -> Self {
        Self { kcal: value }
    }
}

pub struct NutritionValue {
    key: String,
    value: NutritionValueType,

    subvalues: Vec<NutritionValue>,
}
