use super::server_repository::Server;
use crate::{
    data::{
        enums::{ConfigStatus, Country},
        schema,
    },
    enums::errors::internal::{to_internal, InternalError},
};
use diesel::prelude::*;
use diesel::{QueryDsl, Queryable, Selectable};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::Config)]
#[diesel(belongs_to(Server))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Config {
    pub id: Uuid,
    pub private_key: String,
    pub user_ip: String,
    pub server_id: Uuid,
    pub status: ConfigStatus,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = schema::Config)]
pub struct NewConfig {
    pub private_key: String,
    pub user_ip: String,
    pub server_id: Uuid,
}

pub async fn create_config(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<(Config, Server), InternalError> {
    let conn = pool.get().await.map_err(to_internal)?;
    let country = country.to_owned();

    let server = conn
        .interact(move |conn| {
            schema::Server::table
                .filter(schema::Server::country.eq(country))
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
            diesel::insert_into(schema::Config::table)
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
            schema::Config::table
                .inner_join(schema::Server::table)
                .filter(schema::Server::country.eq(country))
                .filter(schema::Config::status.eq(ConfigStatus::NotInUse))
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
