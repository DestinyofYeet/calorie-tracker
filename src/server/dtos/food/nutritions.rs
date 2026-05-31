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
    Percent(f64),
}

pub struct NutritionEnergy {}

pub struct NutritionValue {
    key: String,
    value: NutritionValueType,

    subvalues: Vec<NutritionValue>,
}
