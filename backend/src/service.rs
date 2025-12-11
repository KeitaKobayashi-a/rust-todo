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
    async fn create_todo(&self, title: String, description: String) -> Result<Vec<Todo>, sqlx::Error>;
    async fn delete_todo(&self, id: Uuid) -> Result<Vec<Todo> ,sqlx::Error>;
    }

#[async_trait]
impl<T: TodoRepository + Send + Sync + Clone> TodoService for TodoUsecase<T> {
    async fn get_all_todos(&self) -> Result<Vec<Todo>, sqlx::Error> {
        self.repository.find_all().await
    }

    async fn get_todo_by_id(&self, id: Uuid) -> Result<Option<Todo>, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    async fn create_todo(&self, title: String, description: String) -> Result<Vec<Todo>, sqlx::Error> {
        let new_todo = Todo::new(title, description);
        self.repository.create(new_todo).await
    }

    async fn delete_todo(&self, id: Uuid) -> Result<Vec<Todo> ,sqlx::Error> {
        self.repository.delete(id).await
    }
}