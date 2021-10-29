#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    if let Some(init_url) = &api::config::instance().init_url {
        if let Some(db_name) = &api::config::instance().init_database {
            log::info!("Connecting to DB as admin");
            let sql = format!("CREATE DATABASE {}", db_name);
            let admin_pool = api::database::build_connection_pool_from_url(&init_url).await?;
            let admin_client = &mut *admin_pool.get().await?;
            log::info!("Connected");
            let _ = admin_client.simple_query(&sql).await;
            log::info!("DB INIT complete");
        }
    }

    log::info!("Connecting to DB");
    let pool = api::database::build_connection_pool().await?;
    let client = &mut *pool.get().await?;
    assert!(!client.is_closed(), "DB connection was closed");
    log::info!("Connected");

    if let Err(err) = api::migrations::run(client).await {
        log::error!("ERROR: {:#?}", err);
        panic!("ERROR MIGRATING")
    }
    log::info!("All Migrations Completed");
    Ok(())
}
