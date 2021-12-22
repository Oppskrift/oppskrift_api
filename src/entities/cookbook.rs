use super::{recipe::Recipe, user::User};

pub struct Cookbook {
    name: String,
    description: String,
    owner: User,
    recipes: Vec<Recipe>,
}
