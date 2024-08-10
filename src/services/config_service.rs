use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::info;

use crate::{
    data::{
        enums::{ConfigStatus, Country},
        models::{Config, Server},
        schema,
    },
    enums::errors::internal::Result,
};

pub async fn get_config_by_country(
    pool: &deadpool_diesel::postgres::Pool,
    country: &Country,
) -> Result<(Config, Server)> {
    let conn = pool.get().await?;
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
        .await??;

    let (config, server) = result.first().unwrap().clone();

    info!("Got config by country: {:?}", country);

    Ok((config, server))
}
