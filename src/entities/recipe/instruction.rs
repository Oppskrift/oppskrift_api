use super::ingredient::Ingredient;

pub struct Instruction {
    text: String,
    ingredients: Option<Vec<Ingredient>>,
}
