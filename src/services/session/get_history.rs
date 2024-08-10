use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    data::{
        enums::{Country, SessionStatus},
        models::{Config, Server, Session},
        schema,
    },
    dto::session::{internal::SessionHistory, request::Params},
    enums::errors::internal::Result,
};

pub async fn get_history(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    params: &Params,
) -> Result<Vec<SessionHistory>> {
    let conn = pool.get().await?;

    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);
    let devices = params.devices.clone();
    let countries = params.countries.clone();
    let user_id = *user_id;

    let data: Vec<(Session, Config, Server)> = conn
        .interact(move |conn| {
            schema::session::table
                .inner_join(schema::config::table.inner_join(schema::server::table))
                .filter(schema::session::status.eq(SessionStatus::Closed))
                .filter(
                    schema::server::country.eq_any(
                        countries.unwrap_or(
                            schema::server::table
                                .select(schema::server::country)
                                .load::<Country>(conn)?,
                        ),
                    ),
                )
                .filter(
                    schema::session::device_id.eq_any(
                        devices.unwrap_or(
                            schema::device::table
                                .select(schema::device::id)
                                .filter(schema::device::user_id.eq(user_id))
                                .load::<Uuid>(conn)?,
                        ),
                    ),
                )
                .limit(limit)
                .offset(offset)
                .select((
                    Session::as_select(),
                    Config::as_select(),
                    Server::as_select(),
                ))
                .load::<(Session, Config, Server)>(conn)
        })
        .await??;

    let history = data.iter().map(|(session, _, server)| SessionHistory {
        id: session.id,
        device_id: session.device_id,
        opened_at: session.opened_at,
        closed_at: session.closed_at.unwrap(),
        duration: (session.closed_at.unwrap() - session.opened_at).num_seconds(),
        country: server.country,
    });

    Ok(history.collect())
}
