#[cfg(feature = "server")]
#[actix_web::main]
async fn main() {
    use actix_files::Files;
    use actix_web::HttpServer;
    use leptos::get_configuration;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use recipe_final_exercise::{app::App, database::Database};

    Database::setup().await.unwrap();

    let config = get_configuration(None).await.unwrap().leptos_options;

    let adress = config.site_addr;
    let routes = generate_route_list(App);

    HttpServer::new(move || {
        actix_web::App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(Files::new("pkg", [&config.site_root, "/pkg"].concat()))
            .leptos_routes(config.clone(), routes.clone(), App)
    })
    .bind(&adress)
    .unwrap()
    .run()
    .await
    .unwrap();
}
