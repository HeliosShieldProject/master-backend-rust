use crate::{
    data::{enums::Country, models::Server, schema},
    enums::errors::internal::{to_internal, InternalError},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::info;

pub async fn get_server_by_country(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<Server, InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let country = country.to_owned();

    conn.interact(move |conn| {
        schema::server::table
            .filter(schema::server::country.eq(country))
            .select(Server::as_select())
            .first(conn)
    })
    .await
    .map_err(|e| {
        info!("Server not found: {}", e);
        e
    })
    .map_err(to_internal)?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => InternalError::Internal,
        _ => InternalError::Internal,
    })
    .map(|server| {
        info!("Got server by country: {}", server.id);
        server
    })
}
