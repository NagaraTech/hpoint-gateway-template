use sea_orm::entity::prelude::*;
use sea_orm::{Database, DbBackend, Statement};
use sea_orm_migration::prelude::*;
use hpoint_gateway_template::db::migration::Migrator;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    setup_db().await.expect("DB Setup error");
}

pub async fn setup_db() -> Result<DatabaseConnection, DbErr> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
    let db = Database::connect(db_url.clone()).await?;
    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
                .await?;
            let url = format!("{}/{}", db_url.clone(), db_name);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
                .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
                .await?;
            let url = format!("{}/{}", db_url.clone(), db_name);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    let schema_manager = SchemaManager::new(&db); // To investigate the schema

    Migrator::up(&db, None).await.expect("fail to migrate");
    assert!(schema_manager.has_table("relay_events").await.expect("can not find table"));
    Ok(db)
}
