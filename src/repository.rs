use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub is_done: bool,
}

#[derive(Clone)]
pub struct Repository {
    pool: sqlx::SqlitePool,
}

impl Repository {
    pub async fn try_init() -> anyhow::Result<Repository> {
        let database_url = std::env::var("DATABASE_URL")?;

        let sqlite_options =
            SqliteConnectOptions::from_str(database_url.as_str())?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect_with(sqlite_options)
            .await?;

        sqlx::migrate!("db/migrations").run(&pool).await?;

        Ok(Repository { pool })
    }

    pub async fn insert(&self, text: String) -> Result<Todo, sqlx::Error> {
        sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todos (text)
            VALUES ($1)
            RETURNING id, text, is_done
            "#,
            text
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let todos = sqlx::query_as!(
            Todo,
            r#"
            SELECT id, text, is_done
            FROM todos
            ORDER BY id DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
            SELECT id, text, is_done
            FROM todos
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn update(&self, todo: Todo) -> Result<Todo, sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE todos
            SET text = $2, is_done = $3
            WHERE id = $1
            "#,
            todo.id,
            todo.text,
            todo.is_done
        )
        .execute(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM todos
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_done(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM todos
            WHERE is_done = 1
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
