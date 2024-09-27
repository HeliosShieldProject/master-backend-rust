use crate::{
    data::models::{Device, Session},
    enums::errors::internal::Error,
};

pub trait SessionBy {
    async fn get_session<'a>(
        &self,
        pool: &'a deadpool_diesel::postgres::Pool,
    ) -> Result<(Session, Device), Error>;
}

pub async fn get_session<T: SessionBy>(
    pool: &deadpool_diesel::postgres::Pool,
    by: T,
) -> Result<(Session, Device), Error> {
    by.get_session(pool).await
}
