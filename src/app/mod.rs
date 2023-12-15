mod login;
mod nav_bar;
mod new_recipe;
mod recipes;

use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::{Route, Router, Routes};

use crate::app::{
    login::{SignInView, SignUpView},
    nav_bar::NavBar,
    new_recipe::NewRecipeView,
    recipes::RecipesView,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet href="pkg/app.css"/>

        <Router>

        <NavBar/>

        <main>

        <Routes>
            <Route path="/signin" view=SignInView/>
            <Route path="/signup" view=SignUpView/>
            <Route path="/recipes" view=RecipesView/>
            <Route path="/new_recipe" view=NewRecipeView/>
            <Route path="/*any" view=|| view!{ <h1> "Not found" </h1>}/>
        </Routes>

        </main>

        </Router>
    }
}
