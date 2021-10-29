use std::env;

pub struct Config {
    pub addr: String,
    pub database_url: String,
    pub init_database: Option<String>,
    pub init_url: Option<String>,
}

lazy_static! {
    static ref CONFIG: Config = {
        let port = env::var("PORT").unwrap_or_else(|_| "5000".to_owned());
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
        let addr: String = format!("{}:{}", host, port);

        let default_cs = "postgresql://postgres:POSTGRESPASS@localhost/DBNAME".to_owned();
        let database_url = env::var("DATABASE_URL").unwrap_or(default_cs);

        let init_database = env::var("INIT_DATABASE").ok().map(|x| x.to_string());
        let init_url = env::var("INIT_URL").ok().map(|x| x.to_string());

        Config {
            addr,
            database_url,
            init_database,
            init_url,
        }
    };
}

pub fn instance() -> &'static Config {
    &CONFIG
}
