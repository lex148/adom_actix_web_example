use crate::database::DbClient;
use crate::errors::Result;
use crate::models::orders::Order;
use actix_web::{get, web::Data, HttpResponse};

#[get("/helloworld")]
pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(true))
}

#[get("/orders")]
pub async fn orders(pool: Data<DbClient>) -> Result<HttpResponse> {
    let client = &*pool.get().await?;
    let min_id: i32 = 30;
    let limit: i64 = 100;
    let offset: i64 = 300;
    let orders = Order::find_where(
        client,
        "id > $1 OFFSET $2 LIMIT $3",
        &[&min_id, &offset, &limit],
    )
    .await?;
    Ok(HttpResponse::Ok().json(orders))
}
