use crate::{
    data::{enums::Country, models::Server, schema},
    enums::errors::internal::{to_internal, InternalError},
    logger::{enums::Services::ServerService, ContextLogger, ResultExt},
};
use diesel::prelude::*;
use diesel::QueryDsl;

const LOG: ContextLogger = ContextLogger::new(ServerService);

pub async fn get_server_by_country(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<Server, InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(ServerService)
        .await?;
    let country = country.to_owned();

    let server = conn
        .interact(move |conn| {
            schema::server::table
                .filter(schema::server::country.eq(country))
                .select(Server::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)
        .log_error(ServerService)
        .await?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => InternalError::Internal,
            _ => InternalError::Internal,
        })?;

    LOG.info(format!("Got server by country: {}", &server.id))
        .await;
    Ok(server)
}
