use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "address")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i32,
  pub full_name: String,
  pub search_text_name: String,
  pub full_code: String,
  pub postal_index: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
