use django_rs::{
    chrono::{DateTime, Utc},
    django_rs_macro::{FromIter, SaveData},
    models::{
        column::{ColumnType, CreateColumn, CreateOptions},
        traits::model::Model,
        ModelIteration,
    },
};

#[derive(FromIter, Debug, SaveData)]
pub struct LoginToken {
    pub id: Option<i64>,
    pub user_id: i64,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

impl Model for LoginToken {
    const TABLE_NAME: &'static str = "LoginTokens";

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
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "token",
                ColumnType::String,
                CreateOptions::default().set_non_nullable(),
            ),
            CreateColumn::new(
                "expires_at",
                ColumnType::Date,
                CreateOptions::default().set_non_nullable(),
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
