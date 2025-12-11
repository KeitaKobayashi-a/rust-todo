use crate::models::Todo;
use crate::interface::TodoRepository;
use crate::db::DbPool;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pub pool: DbPool
    }

impl TodoRepositoryImpl{
    pub fn new(pool: DbPool) -> Self{
        Self{pool}
        }
    }

#[async_trait]
impl TodoRepository for TodoRepositoryImpl{
    async fn find_all(&self) -> Result<Vec<Todo>, sqlx::Error>{
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, title, description, completed, created_at, updated_at FROM todos")
            .fetch_all(&self.pool)
            .await?;
            Ok(todos)
        }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, sqlx::Error>{
        let todo = sqlx::query_as::<_, Todo>(
            "SELECT id, title, description, completed, created_at, updated_at FROM todos WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
            Ok(todo)
        }

    async fn create(&self, todo: Todo) -> Result<Todo, sqlx::Error> {
        let created_todo = sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (id, title, description, completed, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, title, description, completed, created_at, updated_at"
        )
            .bind(todo.id)
            .bind(&todo.title)
            .bind(&todo.description)
            .bind(todo.completed)
            .bind(todo.created_at)
            .bind(todo.updated_at)
            .fetch_one(&self.pool)
            .await?;
        Ok(created_todo)
    }

    async fn delete(&self, id: Uuid) -> Result<Vec<Todo>, sqlx::Error> {
        sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, title, description, completed, created_at, updated_at FROM todos")
            .fetch_all(&self.pool)
            .await?;
        Ok(todos)

    }

}
