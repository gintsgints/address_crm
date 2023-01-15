use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Address::Table)
          .if_not_exists()
          .col(ColumnDef::new(Address::Id).big_integer().not_null())
          .col(ColumnDef::new(Address::Fullname).string().not_null())
          .col(ColumnDef::new(Address::SearchTextName).string().not_null())
          .col(ColumnDef::new(Address::FullCode).string().not_null())
          .col(ColumnDef::new(Address::PostalIndex).string())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
      manager
          .drop_table(Table::drop().table(Address::Table).to_owned())
          .await
  }
}

#[derive(Iden)]
enum Address {
    Table,
    Id,
    Fullname,
    SearchTextName,
    FullCode,
    PostalIndex,
}
