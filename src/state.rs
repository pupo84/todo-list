use super::todo::database::memory::TodoMemoryRepository;
use super::todo::database::spec::TodoRepository;
use std::sync::Arc;
use std::sync::Mutex;

pub struct AppState {
    pub data: Arc<Mutex<dyn TodoRepository>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
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
