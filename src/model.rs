#[cfg(feature = "server")]
use bson::oid::ObjectId;

use bson::DateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
    pub admin: bool,
    pub tokens: Vec<Token>,
    pub recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token: String,
    pub expiration: DateTime,
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub instructions: Vec<String>,
    pub icon_url: String,
    pub price_level: u8,
    pub healthy_level: u8,
    pub comment: Vec<Comment>,
    pub notes: Vec<Note>,
    pub ingredients: Vec<Ingredient>,
    pub tools: Vec<Tool>,
    pub categories: Vec<ObjectId>,
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub content: String,
    pub date: DateTime,
    pub user: ObjectId,
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub note: u8,
    pub user: ObjectId,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Ingredient {
    pub name: String,
    pub icon_url: String,
    pub quantity: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tool {
    pub name: String,
    pub icon_url: String,
}

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub name: String,
}
