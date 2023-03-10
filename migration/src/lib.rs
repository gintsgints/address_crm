pub use sea_orm_migration::prelude::*;

mod m20230115_1120_create_address_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230115_1120_create_address_table::Migration)]
    }
}
