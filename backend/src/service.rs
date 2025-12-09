use crate::models::Todo;
use crate::interface::TodoRepository;
use async_trait::async_trait;
use uuid:: Uuid;

#[derive(Clone)]
pub struct TodoUsecase<T: TodoRepository + Clone> {
    repository: T,
    }

impl<T: TodoRepository + Clone> TodoUsecase<T>{
    pub fn new(repository: T) -> Self{
        Self{repository}}
    }

#[async_trait]
pub trait TodoService{
    async fn get_all_todos(&self) -> Result<Vec<Todo>, sqlx::Error>;
    async fn get_todo_by_id(&self, id: Uuid) -> Result<Option<Todo>, sqlx::Error>;
    }