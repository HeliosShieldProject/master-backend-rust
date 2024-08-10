use crate::{
    data::models::{Config, Device, Server, Session},
    enums::errors::internal::Error,
};

pub trait SessionBy {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device, Config, Server), Error>;
}

pub async fn get_session<T: SessionBy>(
    pool: &deadpool_diesel::postgres::Pool,
    by: T,
) -> Result<(Session, Device, Config, Server), Error> {
    by.get_session(pool).await
}
