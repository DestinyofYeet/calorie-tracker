use django_rs::{
    django_rs_macro::{FromIter, SaveData},
    models::{
        column::{ColumnType, CreateColumn, CreateOptions},
        traits::model::Model,
        ModelIteration,
    },
};

#[derive(Debug, SaveData, FromIter)]
pub struct ConsumableInfo {
    pub id: Option<id>,
    pub name: String,
}

impl Model for ConsumableInfo {
    const TABLE_NAME: &'static str = "consumables";

    fn get_migration() -> Vec<django_rs::models::ModelIteration> {
        vec![ModelIteration::Create(vec![
            CreateColumn::new(
                "id",
                ColumnType::Integer,
                CreateOptions::default().set_primary_key(),
            ),
            CreateColumn::new(
                "name",
                ColumnType::String,
                CreateOptions::default().set_non_nullable().set_unique(),
            ),
        ])]
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
}

pub struct ConsumableBaseNutritions {
    pub id: Option<i64>,
    pub consumable_id: i64,
    pub energy: i64,
}
