use crate::backend::model::Todo;
use async_trait::async_trait;
use std::io::Error;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&mut self, title: &str) -> Todo;
    async fn get(&mut self) -> Vec<Todo>;
    async fn get_by_id(&mut self, id: uuid::Uuid) -> Option<Todo>;
    async fn update(
        &mut self,
        id: uuid::Uuid,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Todo, Error>;
    async fn delete(&mut self, id: uuid::Uuid) -> Result<Todo, Error>;
}
