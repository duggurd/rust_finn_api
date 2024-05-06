use tokio;

pub mod search_keys;
pub mod params;
pub mod api;
pub mod response;

use response::ApiResponse;

use sqlx::{sqlite,Connection, Executor};
use sqlx::types::Json;
use sqlx::Row;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = sqlite::SqliteConnection::connect(
        "sqlite:"
        // &std::env::var("DATABASE_URL")?
    ).await?;

    println!("{}", std::env::var("DATABASE_URL")?);

    let data = tokio::fs::read_to_string("data.json").await?;
    let resp = serde_json::from_str::<ApiResponse>(&data)?;
    

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS test (id int);"
    ).execute(&mut conn).await?;

    sqlx::query(
        "INSERT INTO test (id) VALUES (1);"
    ).execute(&mut conn).await?;

    let rows = sqlx::query(
        "SELECT * FROM test;"
    ).fetch_all(&mut conn).await?;

    for row in rows {
        let id: i32 = row.get("id");
        println!("id: {}", id);
    }

    // let a = &resp.docs[0];

    // println!("Type: {}", a._type);
    Ok(())
}

