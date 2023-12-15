use futures::future::OptionFuture;
use leptos::{
    component, create_action, create_rw_signal, event_target_value, server, view, For, IntoView,
    ServerFnError, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate,
};

use crate::{
    cookies::{Cookies, Credentials},
    model::{Ingredient, Tool},
    reactive_vec::ReactiveVec,
};

#[component]
pub fn NewRecipeView() -> impl IntoView {
    let error = create_rw_signal(None);

    let name = create_rw_signal(String::new());
    let icon_url = create_rw_signal(String::new());
    let price_level = create_rw_signal(0);
    let healthy_level = create_rw_signal(0);

    let instructions = ReactiveVec::<String>::new();
    let ingredients = ReactiveVec::<Ingredient>::new();
    let tools = ReactiveVec::<Tool>::new();

    let create_action = create_action(move |()| {
        error.set(None);

        let Ok(Credentials {
            name: user_name,
            token: user_token,
        }) = Cookies::credentials()
        else {
            error.set(Some("You must be signed in to create a recipe"));
            return OptionFuture::from(None);
        };

        let name = name.get_untracked();
        let icon_url = icon_url.get_untracked();
        let price_level = price_level.get_untracked();
        let healthy_level = healthy_level.get_untracked();
        let instructions = instructions.get_values_untracked();
        let ingredients = ingredients.get_values_untracked();
        let tools = tools.get_values_untracked();

        OptionFuture::from(Some(async move {
            match new_recipe(
                user_name,
                user_token,
                name,
                icon_url,
                price_level,
                healthy_level,
                instructions,
                ingredients,
                tools,
            )
            .await
            {
                Ok(true) => {}
                Ok(false) => error.set(Some("Session expired, please sign in")),
                Err(ServerFnError::Request(_)) => error.set(Some("Network error")),
                Err(_) => error.set(Some("Internal error, retry later")),
            };
        }))
    });

    view! {
        <div style="display:flex;flex-direction:row;">
            <p> "Name" </p>
            <input on:input=move |ev| name.set(event_target_value(&ev))/>
        </div>
        <div style="display:flex;flex-direction:row;">
            <p> "Icon url" </p>
            <input on:input=move |ev| icon_url.set(event_target_value(&ev))/>
        </div>
        <div style="display:flex;flex-direction:row;">
            <p> "Price level" </p>
            <input type="range" min=0 max=4 on:input=move |ev| price_level.set(event_target_value(&ev).parse::<u8>().unwrap())/>
        </div>
        <div style="display:flex;flex-direction:row;">
            <p> "Healthy level" </p>
            <input type="range" min=0 max=4 on:input=move |ev| healthy_level.set(event_target_value(&ev).parse::<u8>().unwrap())/>
        </div>
        <h3> "Instructions" </h3>
        <div>
            <For
                each=move || instructions.get()
                key=|instruction|instruction.0
                let:instruction
            >
                <textarea on:input=move |ev| instruction.1.set(event_target_value(&ev))/>
            </For>
            <button on:click=move |_| instructions.push(String::new())> "+" </button>
        </div>
        <h3> "Ingredients" </h3>
        <div>
            <For
                each=move || ingredients.get()
                key=|ingredient| ingredient.0
                let:ingredient
            >

                <div style="display:flex;flex-direction:row;">
                    <p> "Name" </p>
                    <input on:input=move |ev| ingredient.1.update(|ingredient| ingredient.name = event_target_value(&ev))/>
                </div>
                <div style="display:flex;flex-direction:row;">
                    <p> "Icon url" </p>
                    <input on:input=move |ev| ingredient.1.update(|ingredient| ingredient.icon_url = event_target_value(&ev))/>
                </div>
                <div style="display:flex;flex-direction:row;">
                    <p> "Quantity" </p>
                    <input on:input=move |ev| ingredient.1.update(|ingredient| ingredient.quantity = event_target_value(&ev))/>
                </div>
            </For>
            <button on:click=move |_| ingredients.push(Ingredient::default())> "+" </button>
        </div>


        <h3> "Tools" </h3>
        <div>
            <For
                each=move || tools.get()
                key=|tool| tool.0
                let:tool
            >

                <div style="display:flex;flex-direction:row;">
                    <p> "Name" </p>
                    <input on:input=move |ev| tool.1.update(|tool| tool.name = event_target_value(&ev))/>
                </div>
                <div style="display:flex;flex-direction:row;">
                    <p> "Icon url" </p>
                    <input on:input=move |ev| tool.1.update(|tool| tool.icon_url = event_target_value(&ev))/>
                </div>
            </For>
            <button on:click=move |_| tools.push(Tool::default())> "+" </button>
        </div>

        {move || error.get()}

        <button on:click=move |_| create_action.dispatch(())> "Create" </button>
    }
}

#[server(NewRecipe, encoding = "Cbor")]
async fn new_recipe(
    user_name: String,
    user_token: String,
    name: String,
    icon_url: String,
    price_level: u8,
    healthy_level: u8,
    instructions: Vec<String>,
    ingredients: Vec<Ingredient>,
    tools: Vec<Tool>,
) -> Result<bool, ServerFnError> {
    use leptos::logging::error;

    use crate::database::{recipes::NewRecipeError, Database};

    let Ok(database) = Database::new().await else {
        return Err(ServerFnError::ServerError("".into()));
    };

    match database
        .new_recipe(
            user_name,
            user_token,
            name,
            icon_url,
            price_level,
            healthy_level,
            instructions,
            ingredients,
            tools,
        )
        .await
    {
        Ok(()) => Ok(true),
        Err(NewRecipeError::Database(err)) => {
            error!("Database error while creating recipe, {:?}", err);
            return Err(ServerFnError::ServerError("".into()));
        }
        Err(NewRecipeError::InvalidCredentials) => Ok(false),
    }
}
