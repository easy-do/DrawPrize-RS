pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240706_000001_create_table;
mod m20240706_000002_create_table;
mod m20240706_000003_create_table;
mod m20240706_000004_create_table;
mod m20240706_000005_create_table;
mod m20240706_000006_create_table;
mod m20240706_000007_create_table;
mod m20240706_000008_create_table;
mod m20240706_000009_create_table;
mod m20240717_000001_create_table;
mod m20240717_000002_create_table;
mod m20240718_000001_create_table;
mod m20240719_000001_create_table;
mod m20240721_000001_create_table;
mod m20240722_000001_create_table;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration),
             Box::new(m20240706_000001_create_table::Migration),
             Box::new(m20240706_000002_create_table::Migration),
             Box::new(m20240706_000003_create_table::Migration),
             Box::new(m20240706_000004_create_table::Migration),
             Box::new(m20240706_000005_create_table::Migration),
             Box::new(m20240706_000006_create_table::Migration),
             Box::new(m20240706_000007_create_table::Migration),
             Box::new(m20240706_000008_create_table::Migration),
             Box::new(m20240706_000009_create_table::Migration),
             Box::new(m20240717_000001_create_table::Migration),
             Box::new(m20240717_000002_create_table::Migration),
             Box::new(m20240718_000001_create_table::Migration),
             Box::new(m20240719_000001_create_table::Migration),
             Box::new(m20240721_000001_create_table::Migration),
             Box::new(m20240722_000001_create_table::Migration),
        ]
    }
}
