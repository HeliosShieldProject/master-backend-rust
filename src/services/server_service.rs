use crate::{
    data::{enums::Country, models::Server, schema},
    enums::errors::internal::InternalError,
};
use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::info;

#[allow(dead_code)]
pub async fn get_server_by_country(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<Server, InternalError> {
    let conn = pool.get().await?;
    let country = country.to_owned();

    let server = conn
        .interact(move |conn| {
            schema::server::table
                .filter(schema::server::country.eq(country))
                .select(Server::as_select())
                .first(conn)
        })
        .await??;

    info!("Got server by country: {}", server.id);

    Ok(server)
}
