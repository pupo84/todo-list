use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTodoRequestDTO {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequestDTO {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponseDTO {
    pub error: String,
}
