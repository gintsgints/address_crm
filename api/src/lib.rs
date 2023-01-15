use migration::{Migrator, MigratorTrait};
use crm_address_core::{
  sea_orm::{Database, DatabaseConnection}
};

#[tokio::main]
pub async fn main() {
  let db_url = "sqlite::memory:";

  let conn:DatabaseConnection = Database::connect(db_url).await.unwrap();
  Migrator::up(&conn, None).await.unwrap();
}
