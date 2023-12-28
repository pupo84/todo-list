mod backend;
mod state;

use actix_cors::Cors;
use actix_web::web;
use actix_web::{web::Data, App, HttpServer};
use backend::controller::{healthcheck, not_found};
use backend::database::postgres::TodoPostgresRespository;
use dotenv::dotenv;
use state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut app_state = AppState::new();
    let should_persist_data = env::var("PERSIST_DATA").unwrap().parse().unwrap_or(false);
    if should_persist_data {
        app_state.assign(TodoPostgresRespository::new());
    }

    let app_data = Data::new(app_state.data);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(backend::controller::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
