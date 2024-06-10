pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240604_145846_create_check_in_events_table;
mod m20240604_145849_create_online_time_events_table;
mod m20240604_152254_create_relay_events_table;
mod m20240610_075537_create_post_data_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240604_145846_create_check_in_events_table::Migration),
            Box::new(m20240604_145849_create_online_time_events_table::Migration),
            Box::new(m20240604_152254_create_relay_events_table::Migration),
            Box::new(m20240610_075537_create_post_data_table::Migration),
        ]
    }
}
