use sea_orm_migration::prelude::*;

use super::m20230115_1120_create_address_table::Address;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_index(
        Index::create()
          .name("search-index")
          .table(Address::Table)
          .col(Address::SearchTextName)
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_index(
        Index::drop()
          .name("search-index")
          .table(Address::Table)
          .to_owned(),
      )
      .await
  }
}
