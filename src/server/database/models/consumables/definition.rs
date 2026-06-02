use django_rs::{
    django_rs_macro::{FromIter, SaveData},
    models::{
        column::{ColumnType, CreateColumn, CreateOptions},
        traits::model::Model,
        ModelIteration,
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    dtos::food::{NutritionEnergy, NutritionValue, NutritionValueType},
    server::database::models::user::UserDB,
};

#[derive(Debug, SaveData, FromIter, Serialize, Deserialize)]
pub struct ConsumableDB {
    pub id: Option<i64>,
    pub user_id: i64,
    pub name: String,

    pub serving_size: NutritionValueType,
    pub energy: NutritionEnergy,
    pub fat: NutritionValueType,
    pub carbohydrates: NutritionValueType,
    pub salt: NutritionValueType,
    pub proteins: NutritionValueType,

    pub extra_values: Vec<NutritionValue>,
}

impl Model for ConsumableDB {
    const TABLE_NAME: &'static str = "Consumables";

    fn get_migration() -> Vec<django_rs::models::ModelIteration> {
        vec![ModelIteration::Create(vec![
            CreateColumn::new(
                "id",
                ColumnType::Integer,
                CreateOptions::default().set_primary_key(),
            ),
            CreateColumn::new(
                "user_id",
                ColumnType::Integer,
                CreateOptions::default().set_non_nullable().set_foreign_key(
                    UserDB::TABLE_NAME,
                    UserDB::get_latest_column_name("id").unwrap(),
                ),
            ),
            CreateColumn::new(
                "name",
                ColumnType::String,
                CreateOptions::default().set_non_nullable().set_unique(),
            ),
            CreateColumn::new(
                "serving_size",
                ColumnType::Json,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "energy",
                ColumnType::Json,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "fat",
                ColumnType::Json,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "carbohydrates",
                ColumnType::Json,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "salt",
                ColumnType::Json,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "proteins",
                ColumnType::Json,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new("extra_values", ColumnType::Json, CreateOptions::default()),
        ])]
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
}
