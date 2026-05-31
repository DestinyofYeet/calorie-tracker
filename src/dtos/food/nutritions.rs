use std::{convert::Infallible, str::FromStr, string::ParseError};

use thiserror::Error;

#[derive(Debug)]
pub struct Nutritions {
    pub energy: NutritionEnergy,
    pub fat: NutritionValueType,
    pub serving_size: NutritionValueType,
    pub carbohydrates: NutritionValueType,
    pub salt: NutritionValueType,
    pub proteins: NutritionValueType,

    pub extra_values: Vec<NutritionValue>,
}

#[derive(Debug)]
pub enum NutritionValueType {
    Gram(f64),
    Kilogram(f64),
}

impl NutritionValueType {
    pub fn get_options() -> Vec<(String, String)> {
        [("g", "gram"), ("kg", "kilogram")]
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

impl TryFrom<(String, f64)> for NutritionValueType {
    type Error = ValueParseError;

    fn try_from(value: (String, f64)) -> Result<Self, Self::Error> {
        let (key, value) = value;
        match key.as_str() {
            "gram" => Ok(Self::Gram(value)),
            "kilogram" => Ok(Self::Kilogram(value)),
            _ => Err(ValueParseError::InvalidKey(key)),
        }
    }
}

impl TryFrom<(String, String)> for NutritionValueType {
    type Error = ValueParseError;

    fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
        let (key, value) = value;

        let value = if let Ok(value) = value.parse::<i64>().map(|e| e as f64) {
            value
        } else {
            value
                .parse::<f64>()
                .map_err(|e: std::num::ParseFloatError| {
                    ValueParseError::InvalidValue(e.to_string())
                })?
        };

        Self::try_from((key, value))
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct NutritionValue {
    key: String,
    value: NutritionValueType,

    subvalues: Vec<NutritionValue>,
}
