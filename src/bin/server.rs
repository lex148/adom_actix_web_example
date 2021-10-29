use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let pool = match api::database::build_connection_pool().await {
        Ok(p) => p,
        Err(err) => panic!("DB CONNECTION POOL ERROR: {}", err),
    };

    let bind_interface = api::config::instance().addr.clone();
    log::info!("Server Running: {}", bind_interface);

    use api::controllers;

    HttpServer::new(move || {
        App::new()
            //.app_data(pool.clone())
            .data(pool.clone())
            .service(controllers::helloworld::index)
            .service(controllers::helloworld::orders)
            .default_service(web::get().to(p404))
    })
    .bind(bind_interface)?
    .run()
    .await
}

pub async fn p404() -> HttpResponse {
    HttpResponse::NotFound().finish()
}
