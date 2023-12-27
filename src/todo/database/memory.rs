use super::spec::TodoRepository;
use crate::todo::model::Todo;
use async_trait::async_trait;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub struct TodoMemoryRepository {
    db: HashMap<String, Todo>,
}

impl TodoMemoryRepository {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

#[async_trait]
impl TodoRepository for TodoMemoryRepository {
    async fn create(&mut self, title: &str) -> Todo {
        let todo = Todo::new(String::from(title));
        self.db.insert(todo.id.to_string(), todo.clone());
        todo
    }

    async fn get(&mut self) -> Vec<Todo> {
        let mut data: Vec<Todo> = Vec::new();
        for (_, value) in self.db.clone().into_iter() {
            data.push(value);
        }
        data
    }

    async fn get_by_id(&mut self, id: uuid::Uuid) -> Option<Todo> {
        self.db.get(&id.to_string()).cloned()
    }

    async fn update(
        &mut self,
        id: uuid::Uuid,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Todo, Error> {
        match self.db.get_mut(&id.to_string()) {
            Some(todo) => {
                let mut updated_todo = todo.clone();
                if let Some(optional_title) = title {
                    updated_todo.title = optional_title;
                }
                if let Some(optional_completed) = completed {
                    updated_todo.completed = optional_completed;
                }
                self.db.insert(id.to_string(), updated_todo.clone());
                Ok(updated_todo)
            }
            None => Err(Error::new(ErrorKind::NotFound, "Todo not found!")),
        }
    }

    async fn delete(&mut self, id: uuid::Uuid) -> Result<Todo, Error> {
        match self.db.remove(&id.to_string()) {
            Some(todo) => Ok(todo),
            None => Err(Error::new(ErrorKind::NotFound, "Todo not found!")),
        }
    }
}
