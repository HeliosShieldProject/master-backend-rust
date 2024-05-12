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
use tracing::info;

use super::server_service;

pub async fn create_config(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<(Config, Server), InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let country = country.to_owned();

    let server = server_service::get_server_by_country(pool, &country).await?;

    let config = NewConfig {
        private_key: "private_key".to_string(),
        user_ip: "user_ip".to_string(),
        server_id: server.id,
    };

    conn.interact(move |conn| {
        diesel::insert_into(schema::config::table)
            .values(&config)
            .get_result::<Config>(conn)
    })
    .await
    .map_err(|e| {
        info!("Failed to create config: {}", e);
        e
    })
    .map_err(to_internal)?
    .map_err(|_| InternalError::Internal)
    .map(|config| {
        info!("Created config: {}", config.id);
        (config, server)
    })
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
        .map_err(|e| {
            info!("Config not found: {}", e);
            e
        })
        .map_err(to_internal)?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => InternalError::Internal,
            _ => InternalError::Internal,
        })?;

    if !result.is_empty() {
        let (config, server) = result.first().unwrap().clone();
        info!("Got config by country: {:?}", country);

        return Ok((config, server));
    }

    Ok(create_config(pool, &country).await?)
}
