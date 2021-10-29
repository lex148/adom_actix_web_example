use crate::errors::Result;
use log::info;
use std::fs::DirEntry;

async fn completed_list(client: &tokio_postgres::Client) -> Result<Vec<String>> {
    let result = client
        .simple_query(
            "
    CREATE TABLE _migrations (
        id          SERIAL PRIMARY KEY,
        completed   TEXT NOT NULL
    )",
        )
        .await;
    if let Ok(_) = result {
        info!("_migrations table created")
    }
    let rows = client
        .query("SELECT completed FROM _migrations", &[])
        .await?;
    let completed: Vec<String> = rows.iter().map(|x| x.get(0)).filter_map(|x| x).collect();
    Ok(completed)
}

async fn migrate(client: &mut tokio_postgres::Client, key: String, sql: String) -> Result<()> {
    let trans = client.transaction().await?;
    for part in sql.split(';') {
        log::debug!("RUNNING: {}", part);
        trans.simple_query(&part).await?;
    }
    trans
        .execute("INSERT INTO _migrations (completed) VALUES ($1)", &[&key])
        .await?;

    trans.commit().await?;
    info!("Migrated: {}", key);
    Ok(())
}

pub async fn run(client: &mut tokio_postgres::Client) -> Result<()> {
    let completed: Vec<String> = completed_list(client).await?;
    let files = std::fs::read_dir("./migrations")?;
    let files: std::result::Result<Vec<DirEntry>, _> = files.collect();
    let mut files = files?;
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    for entity in files {
        let key: String = format!("{:?}", entity.file_name());
        if !completed.contains(&key) {
            let content = std::fs::read_to_string(entity.path())?;
            migrate(client, key, content).await?;
        }
    }
    Ok(())
}
