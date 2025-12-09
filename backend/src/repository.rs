use crate::models:Todo;
use crate::interface::ToDoRepository;
use crate::db::DbPool;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct ToDoRepositoryImpl {
    pub pool: DbPool
    }

impl ToDoRepositoryImpl{
    pub fn new(pool: DbPool) -> Self{
        Self{pool}
        }
    }

#[async_trait]
impl ToDoRepository for ToDoRepositoryImpl{
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
    }
