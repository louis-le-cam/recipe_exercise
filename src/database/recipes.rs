use bson::{doc, to_bson};
use mongodb::{error::Error as MongoError, options::FindOptions};
use thiserror::Error;

use crate::model::{Ingredient, Recipe, Tool};

use super::Database;

#[derive(Error, Debug)]
pub enum GetRecipesError {
    #[error(transparent)]
    Database(#[from] MongoError),
}

#[derive(Error, Debug)]
pub enum NewRecipeError {
    #[error(transparent)]
    Database(#[from] MongoError),
    #[error("Invalid credentials")]
    InvalidCredentials,
}

impl Database {
    pub async fn get_recipes(&self, limit: usize) -> Result<Vec<Recipe>, GetRecipesError> {
        let users = self.users();

        let mut users = users.find(None, FindOptions::builder().build()).await?;

        let mut recipes = Vec::with_capacity(limit);

        while recipes.len() < limit {
            if !users.advance().await? {
                break;
            };

            let user = users.deserialize_current()?;

            recipes.extend_from_slice(&user.recipes);
        }

        if recipes.len() > limit {
            recipes = recipes.into_iter().take(limit).collect();
        }

        Ok(recipes)
    }

    pub async fn new_recipe(
        &self,
        user_name: String,
        user_token: String,
        name: String,
        icon_url: String,
        price_level: u8,
        healthy_level: u8,
        instructions: Vec<String>,
        ingredients: Vec<Ingredient>,
        tools: Vec<Tool>,
    ) -> Result<(), NewRecipeError> {
        let users = self.users();

        let Some(user) = users.find_one(doc! {"name": &user_name}, None).await? else {
            return Err(NewRecipeError::InvalidCredentials);
        };

        if user
            .tokens
            .iter()
            .find(|token| token.token == user_token)
            .is_none()
        {
            return Err(NewRecipeError::InvalidCredentials);
        }

        let recipe = Recipe {
            name,
            instructions,
            icon_url,
            price_level,
            healthy_level,
            comment: Vec::new(),
            notes: Vec::new(),
            ingredients,
            tools,
            categories: Vec::new(),
        };

        users
            .update_one(
                doc! { "name": &user_name},
                doc! { "$push": {"recipes": to_bson(&recipe).unwrap()}},
                None,
            )
            .await?;

        Ok(())
    }
}
