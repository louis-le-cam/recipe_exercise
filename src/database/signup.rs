use bcrypt::DEFAULT_COST;
use leptos::logging::error;
use mongodb::error::Error as MongoError;
use thiserror::Error;

use crate::model::{Token, User};

use super::{generate_token, Database};

#[derive(Error, Debug)]
pub enum SignupError {
    #[error(transparent)]
    Database(#[from] MongoError),
    #[error("Name already taken")]
    NameAlreadyTaken,
    #[error("Internal error")]
    Internal,
}

impl Database {
    pub async fn signup(&self, name: String, password: String) -> Result<Token, SignupError> {
        let users = self.users();

        let token = generate_token();

        let hashed_password = match bcrypt::hash(password, DEFAULT_COST) {
            Ok(ok) => ok,
            Err(err) => {
                error!("Failed to hash password, {:?}", err);
                return Err(SignupError::Internal);
            }
        };

        let user = User {
            name,
            password: hashed_password,
            admin: false,
            tokens: vec![token.clone()],
            recipes: Vec::new(),
        };

        users.insert_one(user, None).await?;

        Ok(token)
    }
}
