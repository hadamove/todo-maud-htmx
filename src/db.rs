use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub status: ItemStatus,
}

#[derive(Clone)]
pub struct Database {
    pool: sqlx::SqlitePool,
}

impl Database {
    pub async fn init_db() -> anyhow::Result<Database> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let sqlite_options =
            SqliteConnectOptions::from_str(database_url.as_str())?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect_with(sqlite_options)
            .await?;

        sqlx::migrate!("db/migrations").run(&pool).await?;

        Ok(Database { pool })
    }

    pub async fn insert(&self, item: Item) -> Result<Item, sqlx::Error> {
        sqlx::query_as!(
            Item,
            r#"
            INSERT INTO items (name, description, status)
            VALUES ($1, $2, $3)
            RETURNING id, name, description, status as "status!: ItemStatus"
            "#,
            item.name,
            item.description,
            item.status
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<Item>, sqlx::Error> {
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT id, name, description, status as "status!: ItemStatus"
            FROM items
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Item, sqlx::Error> {
        let item = sqlx::query_as!(
            Item,
            r#"
            SELECT id, name, description, status as "status!: ItemStatus"
            FROM items
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn update(&self, item: Item) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE items
            SET name = $2, description = $3, status = $4
            WHERE id = $1
            "#,
            item.id,
            item.name,
            item.description,
            item.status
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM items
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
