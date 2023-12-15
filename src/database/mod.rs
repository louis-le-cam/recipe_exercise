#![cfg(feature = "server")]

pub mod recipes;
pub mod signin;
pub mod signup;

use std::cell::RefCell;

use base64::{engine::general_purpose::STANDARD, Engine};
use leptos::logging::error;
use mongodb::{
    bson::{doc, DateTime},
    options::IndexOptions,
    Client, Collection, Database as MongoDatabase, IndexModel,
};
use rand::{thread_rng, Rng};

use crate::model::{Token, User};

thread_local! {
    pub static CLIENT: RefCell<Option<Client>> = RefCell::new(None);
}

/// Wrapper around mongodb's database implementing
/// all the function of the project database
pub struct Database {
    database: MongoDatabase,
}
impl Database {
    const URI: &'static str = "mongodb://localhost:27017";
    const NAME: &'static str = "recipe_final_exercise";

    /// Get the thread local database or create one
    pub async fn new() -> Result<Self, ()> {
        let client = match CLIENT.with_borrow(|client| client.clone()) {
            Some(some) => some,
            None => {
                let new_client = match Client::with_uri_str(Self::URI).await {
                    Ok(ok) => ok,
                    Err(err) => {
                        error!("Failed to create mongodb client {:?}", err);
                        return Err(());
                    }
                };
                CLIENT.with_borrow_mut(|client| *client = Some(new_client.clone()));
                new_client
            }
        };

        Ok(Self {
            database: client.database(Self::NAME),
        })
    }

    /// Setup the database
    /// Should be called once at start of the program
    /// Setup mongodb indices
    pub async fn setup() -> Result<(), ()> {
        let Ok(database) = Self::new().await else {
            return Err(());
        };

        database
            .users()
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "name": 1 })
                    .options(Some(IndexOptions::builder().unique(true).build()))
                    .build(),
                None,
            )
            .await
            .map(|_| ())
            .map_err(|err| {
                error!("Failed to create mongodb unique indices, {:?}", err);
            })
    }

    /// Get the user collection
    fn users(&self) -> Collection<User> {
        self.database.collection("users")
    }
}

/// Generate a random 32 bytes token for use as authentification token
fn generate_token() -> Token {
    Token {
        token: STANDARD.encode(thread_rng().gen::<[u8; 32]>()),
        expiration: DateTime::from_millis(
            DateTime::now().timestamp_millis() + 1000 * 60 * 60 * 24 * 30,
        ),
    }
}
