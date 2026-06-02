use django_rs::{
    models::search::SearchQuery,
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError},
};

use crate::{
    dtos::food::Consumable,
    server::{
        database::models::{consumables::ConsumableDB, user::UserDB},
        entry::SERVER,
    },
};

impl ConsumableDB {
    pub fn get_consumable(
        id: Option<i64>,
        user: &UserDB,
    ) -> Result<Vec<Consumable>, DatabaseStrategyError> {
        let db = SERVER.get_database();
        let mut search_query = SearchQuery::empty().add_constraint(("user_id", user.id.unwrap()));

        if let Some(id) = id {
            search_query = search_query.add_constraint(("id", id));
        }

        let models = db.search_multiple_model::<Self>(&db.get_connection(), search_query)?;

        let result = models.into_iter().map(Into::into).collect();

        Ok(result)
    }
}
