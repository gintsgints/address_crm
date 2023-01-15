use std::env;

use salvo::prelude::*;
use migration::{Migrator, MigratorTrait};
use salvo::extra::affix;
use crm_address_core::{
  sea_orm::{Database, DatabaseConnection},
  Query
};

const DEFAULT_POSTS_PER_PAGE: u64 = 5;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[handler]
async fn list(req: &mut Request, depot: &mut Depot, res: &mut Response) {
  let state = depot
  .obtain::<AppState>()
  .ok_or_else(StatusError::internal_server_error).unwrap();

  let conn = &state.conn;

  let page = req.query("page").unwrap_or(1);
  let posts_per_page = req
    .query("posts_per_page")
    .unwrap_or(DEFAULT_POSTS_PER_PAGE);
  let search = req.query::<String>("search").unwrap();


  let (posts, _num_pages) = Query::find_address_in_page(conn, search, page, posts_per_page)
    .await
    .map_err(|_| StatusError::internal_server_error()).unwrap();

  res.render(serde_json::to_string(&posts).unwrap())
}

#[tokio::main]
pub async fn main() {
  dotenvy::dotenv().ok();
  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
  let host = env::var("HOST").expect("HOST is not set in .env file");
  let port = env::var("PORT").expect("PORT is not set in .env file");
  let server_url = format!("{}:{}", host, port);

  let conn:DatabaseConnection = Database::connect(db_url).await.unwrap();
  Migrator::up(&conn, None).await.unwrap();

  println!("Starting server at {}", server_url);

  let state = AppState { conn };

  let router = Router::new()
    .hoop(affix::inject(state))
    .get(list);

  Server::new(TcpListener::bind(&format!("{}:{}", host, port)))
    .serve(router)
    .await;
}
