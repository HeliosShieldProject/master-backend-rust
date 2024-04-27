use crate::{
    data::{
        enums::{ConfigStatus, Country},
        models::{Config, Server},
        schema,
    },
    dto::config::internal::NewConfig,
    enums::errors::internal::{to_internal, InternalError},
};
use diesel::prelude::*;
use diesel::QueryDsl;

pub async fn create_config(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<(Config, Server), InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let country = country.to_owned();

    let server = conn
        .interact(move |conn| {
            schema::server::table
                .filter(schema::server::country.eq(country))
                .select(Server::as_select())
                .first(conn)
        })
        .await
        .map_err(to_internal)?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => InternalError::Internal,
            _ => InternalError::Internal,
        })?;

    let config = NewConfig {
        private_key: "private_key".to_string(),
        user_ip: "user_ip".to_string(),
        server_id: server.id,
    };

    let config = conn
        .interact(move |conn| {
            diesel::insert_into(schema::config::table)
                .values(&config)
                .get_result::<Config>(conn)
        })
        .await
        .map_err(to_internal)?
        .map_err(|_| InternalError::Internal)?;

    Ok((config, server))
}

pub async fn get_config_by_country(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<(Config, Server), InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let country = country.to_owned();

    let result: Vec<(Config, Server)> = conn
        .interact(move |conn| {
            schema::config::table
                .inner_join(schema::server::table)
                .filter(schema::server::country.eq(country))
                .filter(schema::config::status.eq(ConfigStatus::NotInUse))
                .select((Config::as_select(), Server::as_select()))
                .load::<(Config, Server)>(conn)
        })
        .await
        .map_err(to_internal)?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => InternalError::Internal,
            _ => InternalError::Internal,
        })?;
    if !result.is_empty() {
        let (config, server) = result.first().unwrap().clone();
        return Ok((config, server));
    }

    Ok(create_config(pool, &country).await?)
}
