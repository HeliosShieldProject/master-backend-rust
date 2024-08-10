use uuid::Uuid;

use super::close_session_by_id;
use crate::{
    dto::session::{interface::get_session, query::ActiveSessionAndDevice},
    enums::errors::internal::Result,
};

pub async fn close_session(
    pool: &deadpool_diesel::postgres::Pool,
    device_id: &Uuid,
) -> Result<Uuid> {
    let (session, _, _, _) = get_session(
        pool,
        ActiveSessionAndDevice {
            device_id: *device_id,
        },
    )
    .await?;
    let session_id = close_session_by_id(pool, &session.id).await?;
    Ok(session_id)
}
