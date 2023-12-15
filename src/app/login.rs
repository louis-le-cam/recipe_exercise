use leptos::{
    component, create_action, create_rw_signal, event_target_value, server, view, Action, IntoView,
    RwSignal, ServerFnError, SignalGet, SignalGetUntracked, SignalSet,
};

use crate::{cookies::Cookies, model::Token};

#[component]
pub fn SignInView() -> impl IntoView {
    let action = create_action(move |args: &LoginActionArgs| {
        let LoginActionArgs {
            name,
            password,
            error,
        } = args.clone();

        error.set(None);

        async move {
            match signin(name.get_untracked(), password.get_untracked()).await {
                Ok(Some(token)) => {
                    if Cookies::set_credentials(
                        &name.get_untracked(),
                        &token.token,
                        &token.expiration,
                    )
                    .is_err()
                    {
                        error.set(Some("Internal error, retry later"));
                    }
                }
                Ok(None) => error.set(Some("Wrong name or password")),
                Err(ServerFnError::Request(_)) => error.set(Some("Network error")),
                Err(_) => error.set(Some("Internal error, retry later")),
            };
        }
    });

    view! {
        <LoginView action=action title="Sign in"/>
    }
}

#[component]
pub fn SignUpView() -> impl IntoView {
    let action = create_action(move |args: &LoginActionArgs| {
        let LoginActionArgs {
            name,
            password,
            error,
        } = args.clone();

        error.set(None);

        async move {
            match signup(name.get(), password.get()).await {
                Ok(Some(token)) => {
                    if Cookies::set_credentials(&name.get(), &token.token, &token.expiration)
                        .is_err()
                    {
                        error.set(Some("Internal error, retry later"));
                    }
                }
                Ok(None) => error.set(Some("Name already taken")),
                Err(ServerFnError::Request(_)) => error.set(Some("Network error")),
                Err(_) => error.set(Some("Internal error, retry later")),
            }
        }
    });

    view! {
        <LoginView action=action title="Sign in"/>
    }
}

#[derive(Clone)]
struct LoginActionArgs {
    name: RwSignal<String>,
    password: RwSignal<String>,
    error: RwSignal<Option<&'static str>>,
}

#[component]
fn LoginView(action: Action<LoginActionArgs, ()>, #[prop(into)] title: String) -> impl IntoView {
    let name = create_rw_signal(String::new());
    let password = create_rw_signal(String::new());
    let error = create_rw_signal(None);

    let submit = move |_| {
        action.dispatch(LoginActionArgs {
            name,
            password,
            error,
        });
    };

    view! {
        <div class="login">
            <div class="login_form">
                <div class="login_input_container">
                    <label for="login_name_input"> "Name" </label>
                    <input
                        on:input=move |ev| {
                            name.set(event_target_value(&ev));
                        }
                        id="login_name_input" class="login_input"
                        type="text" maxLength=256 required
                    />
                </div>

                <div class="login_input_container">
                    <label for="login_password_input"> "Password" </label>
                    <input
                        on:input=move |ev| {
                            password.set(event_target_value(&ev));
                        }
                        id="login_password_input" class="login_input"
                        type="password" minLength=8 maxLength=256 required
                    />
                </div>

                <span class="login_error"> {move || error.get()} </span>

                <button on:click=submit> {title} </button>
            </div>
        </div>
    }
}

#[server(Signin, encoding = "Cbor")]
async fn signin(name: String, password: String) -> Result<Option<Token>, ServerFnError> {
    use leptos::logging::error;

    use crate::database::{signin::SigninError, Database};

    let Ok(database) = Database::new().await else {
        return Err(ServerFnError::ServerError("".into()));
    };

    match database.signin(name, password).await {
        Ok(token) => Ok(Some(token)),
        Err(SigninError::Database(err)) => {
            error!("Database error while signin: {:?}", err);
            Err(ServerFnError::ServerError("".into()))
        }
        Err(SigninError::WrongNameOrPassword) => Ok(None),
        Err(SigninError::Internal) => Err(ServerFnError::ServerError("".into())),
    }
}

#[server(Signup, encoding = "Cbor")]
async fn signup(name: String, password: String) -> Result<Option<Token>, ServerFnError> {
    use leptos::logging::error;

    use crate::database::{signup::SignupError, Database};

    let Ok(database) = Database::new().await else {
        return Err(ServerFnError::ServerError("".into()));
    };

    match database.signup(name, password).await {
        Ok(token) => Ok(Some(token)),
        Err(SignupError::NameAlreadyTaken) => Ok(None),
        Err(SignupError::Database(err)) => {
            error!("Database error while signup: {:?}", err);
            Err(ServerFnError::ServerError("".into()))
        }
        Err(SignupError::Internal) => Err(ServerFnError::ServerError("".into())),
    }
}
