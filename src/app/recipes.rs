use leptos::{component, server, view, Await, CollectView, IntoView, ServerFnError};
use serde::{Deserialize, Serialize};

#[component]
pub fn RecipesView() -> impl IntoView {
    view! {
        <h2> "Recipes" </h2>
        <Await
            future=move || get_recipes()
            let:recipes
        >
            {
                match recipes.as_ref() {
                    Ok(recipes) => {
                        recipes.iter().map(|recipe| {
                            view! { <p> {&recipe.name} </p> }
                        }).collect_view()
                    },
                    Err(_) => {
                        view! { <p> "Failed to query recipes" </p> }.into_view()
                    },
                }
            }
        </Await>
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecipeInfo {
    name: String,
    icon_url: String,
}

#[server(GetRecipes, encoding = "GetCbor")]
async fn get_recipes() -> Result<Vec<RecipeInfo>, ServerFnError> {
    use leptos::logging::error;

    use crate::database::{recipes::GetRecipesError, Database};

    let Ok(database) = Database::new().await else {
        return Err(ServerFnError::ServerError("".into()));
    };

    match database.get_recipes(50).await {
        Ok(recipes) => Ok(recipes
            .into_iter()
            .map(|recipe| RecipeInfo {
                name: recipe.name,
                icon_url: recipe.icon_url,
            })
            .collect()),
        Err(GetRecipesError::Database(err)) => {
            error!("Database error while getting recipes, {:?}", err);
            Err(ServerFnError::ServerError("".into()))
        }
    }
}
