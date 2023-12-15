use bson::{doc, to_bson};
use leptos::logging::error;
use mongodb::error::Error as MongoError;
use thiserror::Error;

use crate::model::Token;

use super::{generate_token, Database};

#[derive(Error, Debug)]
pub enum SigninError {
    #[error(transparent)]
    Database(#[from] MongoError),
    #[error("Wrong name or password")]
    WrongNameOrPassword,
    #[error("Internal error")]
    Internal,
}

impl Database {
    pub async fn signin(&self, name: String, password: String) -> Result<Token, SigninError> {
        let users = self.users();

        let Some(user) = users.find_one(doc! {"name": &name}, None).await? else {
            return Err(SigninError::WrongNameOrPassword);
        };

        match bcrypt::verify(password, &user.password) {
            Ok(true) => {}
            Ok(false) => return Err(SigninError::WrongNameOrPassword),
            Err(err) => {
                error!("Failed to verify password: {:?}", err);
                return Err(SigninError::Internal);
            }
        }

        let token = generate_token();

        users
            .update_one(
                doc! {"name": &name},
                doc! {"$push": doc! { "tokens": to_bson(&token).unwrap() }},
                None,
            )
            .await?;

        Ok(token)
    }
}
