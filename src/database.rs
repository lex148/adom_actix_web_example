use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;
use tokio_postgres::NoTls;

pub type DbClient = Pool<PostgresConnectionManager<NoTls>>;

pub async fn build_connection_pool() -> Result<DbClient, tokio_postgres::Error> {
    let local_settings = crate::config::instance();
    let database_url = &local_settings.database_url;
    let config = tokio_postgres::config::Config::from_str(database_url).unwrap();
    let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);
    let pool: Pool<_> = Pool::builder().build(pg_mgr).await?;
    Ok(pool)
}

pub async fn build_connection_pool_from_url(
    database_url: &str,
) -> Result<DbClient, tokio_postgres::Error> {
    let config = tokio_postgres::config::Config::from_str(database_url).unwrap();
    let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);
    let pool: Pool<_> = Pool::builder().build(pg_mgr).await?;
    Ok(pool)
}
