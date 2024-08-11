use axum::{
    extract::{Query, State},
    response::Redirect,
};
use deadpool_diesel::postgres::Pool;

use crate::{dto::auth::request::ConfirmEmailQuery, services::email};

pub async fn confirm_email(
    State(pool): State<Pool>,
    Query(payload): Query<ConfirmEmailQuery>,
) -> Redirect {
    match email::confirm(&pool, &payload.token).await {
        Ok(_) => {
            println!("Email confirmed successfully");
            Redirect::to("https://helios.michkoff.com")
        }
        Err(_) => {
            println!("Email confirmation failed");
            Redirect::to("https://helios.michkoff.com")
        }
    }
}
