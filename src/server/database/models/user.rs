use std::sync::LazyLock;

use django_rs::{
    django_rs_macro::{FromIter, SaveData},
    models::{
        column::{ColumnType, CreateColumn, CreateOptions},
        traits::model::Model,
        MigrationKind, ModelMigration,
    },
};

#[derive(Clone, FromIter, Debug, SaveData)]
pub struct UserDB {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
}

impl Model for UserDB {
    const TABLE_NAME: &'static str = "User";

    fn get_migration() -> &'static Vec<ModelMigration> {
        static MIGRATIONS: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
                0,
                MigrationKind::Create(vec![
                    CreateColumn::new(
                        "id",
                        ColumnType::Integer,
                        CreateOptions::default().set_primary_key(),
                    ),
                    CreateColumn::new(
                        "name",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "email",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable().set_unique(),
                    ),
                    CreateColumn::new(
                        "hashed_password",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                ]),
            )]
        });

        &MIGRATIONS
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }
}
