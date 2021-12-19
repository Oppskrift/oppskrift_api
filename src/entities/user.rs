use crate::entities::cookbook::Cookbook;

pub struct User {
    name: String,
    email: String,
    friends: Vec<User>,
    cookbooks: Vec<Cookbook>,
}
