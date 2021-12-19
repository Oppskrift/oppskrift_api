mod ingredient;
mod instruction;

use self::{ingredient::Ingredient, instruction::Instruction};
use super::{cookbook::Cookbook, user::User};
use std::time::{Duration, Instant};

enum Cost {
    Cheap,
    Average,
    Pricey,
}

pub struct Recipe {
    name: String,
    cook_time: Duration,
    category: String,
    cuisine: String, // cuisine type : French, Japanese, Vegan, etc.
    ingredients: Vec<Ingredient>,
    instructions: Vec<Instruction>,
    yields: u32,
    cost: Cost,
    preparation_time: Duration,
    tools: Option<Vec<String>>,
    total_time: Duration,
    author: User,
    language: String,
    description: String,
    cookbooks: Option<Vec<Cookbook>>,
    created_at: Instant,
    updated_at: Option<Instant>,
}
