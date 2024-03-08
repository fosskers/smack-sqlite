use gumdrop::{Options, ParsingStyle};
use sqlx::{prelude::FromRow, sqlite::SqlitePoolOptions};
use std::time::Duration;

/// Hit an SQLite database.
#[derive(Debug, Options)]
struct Args {
    /// The identity of this executable at runtime.
    #[options(required)]
    id: u32,
}

#[derive(Debug, FromRow)]
struct Row {
    id: u32,
    exec: u32,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = Args::parse_args_or_exit(ParsingStyle::AllOptions);
    println!("Binary ID: {}", args.id);

    tokio::time::sleep(Duration::from_secs(3)).await;

    println!("Going!");

    let pool = SqlitePoolOptions::new()
        .max_connections(6)
        .connect("test.db")
        .await?;

    println!(
        "Acquire Timeout: {}",
        pool.options().get_acquire_timeout().as_millis()
    );

    let res = sqlx::query_as::<_, Row>("select * from smack")
        .fetch_all(&pool)
        .await?;

    println!("Before: {}", res.len());

    for _ in 0..1000 {
        let pule = pool.clone();

        tokio::spawn(async move {
            if let Err(e) = sqlx::query("insert into smack (exec) values (?)")
                .bind(args.id)
                .execute(&pule)
                .await
            {
                eprintln!("Error: {}", e);
            }
        });
    }

    tokio::time::sleep(Duration::from_secs(10)).await;

    let res = sqlx::query_as::<_, Row>("select * from smack")
        .fetch_all(&pool)
        .await?;

    println!("After: {}", res.len());

    Ok(())
}
