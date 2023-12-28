use super::database::spec::TodoRepository;
use super::dto::{CreateTodoRequestDTO, ErrorResponseDTO, UpdateTodoRequestDTO};
use actix_web::web;
use actix_web::{
    delete, get,
    http::StatusCode,
    patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder, Result,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[post("/todos")]
pub async fn create(
    repo: Data<Arc<Mutex<dyn TodoRepository>>>,
    body: Json<CreateTodoRequestDTO>,
) -> impl Responder {
    let mut db = repo.lock().unwrap();
    let id = db.create(&body.into_inner().title).await;
    HttpResponse::Created().json(id)
}

#[get["/todos"]]
pub async fn get(repo: Data<Arc<Mutex<dyn TodoRepository>>>) -> impl Responder {
    let mut db = repo.lock().unwrap();
    let todos = db.get().await;
    HttpResponse::Ok().json(todos)
}

#[get["/todos/{id}"]]
pub async fn get_by_id(
    repo: Data<Arc<Mutex<dyn TodoRepository>>>,
    id: Path<String>,
) -> impl Responder {
    let mut db = repo.lock().unwrap();
    if uuid::Uuid::parse_str(id.as_str()).is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponseDTO {
            error: String::from(format!("Could not parse uuid {}", id.as_str())),
        });
    }
    let uuid = uuid::Uuid::parse_str(id.as_str()).unwrap();
    match db.get_by_id(uuid).await {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().json(ErrorResponseDTO {
            error: String::from("Could not find todo"),
        }),
    }
}

#[patch("/todos/{id}")]
pub async fn update(
    repo: web::Data<Arc<Mutex<dyn TodoRepository>>>,
    id: web::Path<String>,
    body: Json<UpdateTodoRequestDTO>,
) -> impl Responder {
    let mut db = repo.lock().unwrap();
    if uuid::Uuid::parse_str(id.as_str()).is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponseDTO {
            error: String::from(format!("Could not parse uuid {}", id.as_str())),
        });
    }

    let uuid = uuid::Uuid::parse_str(id.as_str()).unwrap();

    let body = body.into_inner();
    let title = body.title;
    let completed = body.completed;
    match db.update(uuid, title, completed).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponseDTO {
            error: String::from("Could not update todo"),
        }),
    }
}

#[delete("/todos/{id}")]
pub async fn delete(
    repo: Data<Arc<Mutex<dyn TodoRepository>>>,
    id: Path<String>,
) -> impl Responder {
    let mut db = repo.lock().unwrap();
    if uuid::Uuid::parse_str(id.as_str()).is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponseDTO {
            error: String::from(format!("Could not parse uuid {}", id.as_str())),
        });
    }
    let uuid: uuid::Uuid = uuid::Uuid::parse_str(id.as_str()).unwrap();
    match db.delete(uuid).await {
        Ok(_) => HttpResponse::new(StatusCode::NO_CONTENT),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponseDTO {
            error: String::from("Could not update todo"),
        }),
    }
}

#[get("/health")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: String::from("Everything is working fine"),
    })
}

pub async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create)
            .service(get)
            .service(get_by_id)
            .service(update)
            .service(delete),
    );
}
