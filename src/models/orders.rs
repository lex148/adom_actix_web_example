use tokio_postgres::GenericClient;

#[derive(Deserialize, Serialize, AdomSelect, AdomUpdate, AdomCreate, AdomDelete)]
#[AdomTable = "orders"]
pub(crate) struct Order {
    pub(crate) id: i32,
    pub(crate) customer: String,
    pub(crate) total: i64,
}
