use std::env;

use ::entity::address::Model;
use crm_address_core::{
    sea_orm::{Database, DatabaseConnection},
    Parser, Query,
};
use migration::{Migrator, MigratorTrait};
use salvo::extra::affix;
use salvo::prelude::*;
use serde::{ser::SerializeMap, Serialize};
use thiserror::Error;

const DEFAULT_POSTS_PER_PAGE: u64 = 5;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error: `{0}`")]
    Status(#[from] salvo::http::StatusError),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AppError::Status(error) => {
                // serializer.serialize_str(&error.name.as_str())
                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("code", error.code.as_str())?;
                map.serialize_entry("name", error.name.as_str())?;
                match &error.summary {
                    Some(s) => map.serialize_entry("summary", s.as_str())?,
                    None => map.serialize_entry("summary", "")?,
                }
                match &error.detail {
                    Some(s) => map.serialize_entry("detail", s.as_str())?,
                    None => map.serialize_entry("detail", "")?,
                }
                map.end()
            }
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let result = AddressResults {
            data: None,
            pagination: None,
            error: Some(self),
        };
        res.render(serde_json::to_string(&result).unwrap());
    }
}

#[derive(Serialize)]
struct Pagination {
    limit: u64,
    page: u64,
    pages: u64,
    total: u64,
}

#[derive(Serialize)]
struct AddressResults {
    data: Option<Vec<Model>>,
    pagination: Option<Pagination>,
    error: Option<AppError>,
}

#[handler]
async fn list(req: &mut Request, depot: &mut Depot, res: &mut Response) -> AppResult<()> {
    let state = depot
        .obtain::<AppState>()
        .ok_or_else(StatusError::internal_server_error)?;

    let conn = &state.conn;

    let page = req.query("page").unwrap_or(1);
    let limit = req.query("limit").unwrap_or(DEFAULT_POSTS_PER_PAGE);
    let search = req
        .query::<String>("search")
        .ok_or(StatusError::bad_request().with_summary("Search query parameter not provided"))?;

    let search_parsed = Parser::build(search);

    let (posts, num_pages_and_items) =
        Query::find_address_in_page(conn, search_parsed, page, limit)
            .await
            .map_err(|_| {
                StatusError::internal_server_error().with_summary("Error executing search")
            })?;

    let result = AddressResults {
        data: Some(posts),
        pagination: Some(Pagination {
            limit,
            page,
            pages: num_pages_and_items.number_of_pages,
            total: num_pages_and_items.number_of_items,
        }),
        error: None,
    };

    res.render(serde_json::to_string(&result).map_err(|_| {
        StatusError::internal_server_error().with_summary("Error parsing query results")
    })?);
    Ok(())
}

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn: DatabaseConnection = Database::connect(db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    println!("Starting server at {}", server_url);

    let state = AppState { conn };

    let router = Router::new().hoop(affix::inject(state)).get(list);

    Server::new(TcpListener::bind(&format!("{}:{}", host, port)))
        .serve(router)
        .await;
}
