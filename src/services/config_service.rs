use crate::{
    data::{
        enums::{ConfigStatus, Country},
        models::{Config, Server},
        schema,
    },
    dto::config::internal::NewConfig,
    enums::errors::internal::{to_internal, InternalError},
    logger::{enums::Services::ConfigService, ContextLogger, ResultExt},
};
use diesel::prelude::*;
use diesel::QueryDsl;

use super::server_service;

const LOG: ContextLogger = ContextLogger::new(ConfigService);

pub async fn create_config(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<(Config, Server), InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(ConfigService)
        .await?;
    let country = country.to_owned();

    let server = server_service::get_server_by_country(pool, &country).await?;

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
        .map_err(to_internal)
        .log_error(ConfigService)
        .await?
        .map_err(|_| InternalError::Internal)?;

    LOG.info(format!("Created config by country: {:?}", country))
        .await;
    Ok((config, server))
}

pub async fn get_config_by_country(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<(Config, Server), InternalError> {
    let conn = pool
        .get()
        .await
        .map_err(to_internal)
        .log_error(ConfigService)
        .await?;
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
        .map_err(to_internal)
        .log_error(ConfigService)
        .await?
        .map_err(|e| match e {
            diesel::result::Error::NotFound => InternalError::Internal,
            _ => InternalError::Internal,
        })?;

    if !result.is_empty() {
        let (config, server) = result.first().unwrap().clone();
        LOG.info(format!("Got config by country: {:?}", country)).await;

        return Ok((config, server));
    }

    Ok(create_config(pool, &country).await?)
}
