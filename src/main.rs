use migration::{Migrator};

mod migration;

#[tokio::main]
async fn main() {
  let db_url = "sqlite::memory:";
  let server_url = "0.0.0.0:7878";

  let conn = sea_orm::Database::connect(db_url).await.unwrap();

  Migrator::up(&conn, None).await.unwrap();

  println!("Hello world");
}
