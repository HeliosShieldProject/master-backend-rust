use crate::{
    data::{
        enums,
        repositories::{device_repository, user_repository},
    },
    dto::auth::Response,
    enums::errors::response::ResponseError,
    utils::{hash::verify_password, token::generate_tokens},
    AppState,
};
use axum::{extract::State, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    password: String,
}

// pub async fn change_password(
//     State(state): State<AppState>,
//     Json(payload): Json<Request>,
// ) -> Result<Json<Response>, ResponseError> 
