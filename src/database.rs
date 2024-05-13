use anyhow::Result;
use sqlx::{pool::Pool, postgres::PgPoolOptions, Postgres};

pub struct Database {
    inside: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let db = PgPoolOptions::new()
            .connect("postgresql://localhost/cofy_server?user=test&password=test")
            .await?;
        Ok(Self { inside: db })
    }

    pub async fn test(&self) -> Result<i64> {
        let row: (i64,) = sqlx::query_as("SELECT * FROM \"test.test_table\";")
            .fetch_one(&self.inside)
            .await?;
        Ok(row.0)
    }
}
