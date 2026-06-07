use django_rs::{
    models::search::SearchQuery,
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError, TransactionOptions},
};

use crate::{
    dtos::consumable::{Consumable, Nutritions},
    server::{
        database::models::{consumables::ConsumableDB, user::UserDB},
        entry::SERVER,
    },
};

impl ConsumableDB {
    pub fn save_consumable(data: Consumable, user: &UserDB) -> Result<(), DatabaseStrategyError> {
        let db = SERVER.get_database();
        let user_id = user.id.unwrap();
        db.with_transaction(|trx| {
            let Consumable {
                id,
                name,
                nutritions,
            } = data;
            let Nutritions {
                energy,
                fat,
                serving_size,
                carbohydrates,
                salt,
                proteins,
                extra_values,
            } = nutritions;

            let mut consumable_db = {
                if let Some(id) = id {
                    match db.search_single_model::<Self>(
                        &db.get_connection(),
                        SearchQuery::empty()
                            .add_constraint(("id", id))
                            .add_constraint(("user_id", user_id)),
                    )? {
                        Some(mut value) => {
                            value.name = name;
                            value.energy = energy;
                            value.fat = fat;
                            value.serving_size = serving_size;
                            value.carbohydrates = carbohydrates;
                            value.salt = salt;
                            value.proteins = proteins;
                            value.extra_values = extra_values;

                            value
                        }
                        None => {
                            return Err(DatabaseStrategyError::Error(format!(
                                "Failed to find consumable with id {}",
                                id
                            )))
                        }
                    }
                } else {
                    Self {
                        id: None,
                        name,
                        user_id,
                        energy,
                        fat,
                        serving_size,
                        carbohydrates,
                        salt,
                        proteins,
                        extra_values,
                    }
                }
            };

            dbg!(&consumable_db);

            db.save_model(&trx, &mut consumable_db)?;

            db.manage_transaction(trx, TransactionOptions::Commit)?;

            Ok(())
        })??;

        Ok(())
    }
}
