mod todo;

use actix_web::web;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use todo::controller::{healthcheck, not_found};
use todo::database::memory::TodoMemoryRepository;
use todo::database::postgres::TodoPostgresRespository;
use todo::database::spec::TodoRepository;

pub struct AppState {
    data: Arc<Mutex<dyn TodoRepository>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            data: Arc::new(Mutex::new(TodoMemoryRepository::new())),
        }
    }

    pub fn assign<T>(&mut self, data: T)
    where
        T: TodoRepository + 'static,
    {
        self.data = Arc::new(Mutex::new(data));
    }
}

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
            .configure(todo::controller::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
