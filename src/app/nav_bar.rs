use leptos::{component, view, IntoView};
use leptos_router::A;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <header>
            <A href="/"> "Home" </A>
            <A href="/signin"> "Sign in" </A>
            <A href="/signup"> "Signup" </A>
            <A href="/recipes"> "Recipes" </A>
            <A href="/new_recipe"> "New Recipe" </A>
        </header>
    }
}
