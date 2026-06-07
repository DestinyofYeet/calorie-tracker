pub enum ConsumptionType {
    Food { consumable_id: i64 },

    Custom { name: String },
}
