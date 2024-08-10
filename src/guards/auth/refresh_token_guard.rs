use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    config::ENV,
    dto::auth::internal::RefreshToken,
    enums::errors::external::{Auth, Error},
    services::device::check_logged_in_device,
    state::AppState,
};

#[async_trait]
impl<S> FromRequestParts<S> for RefreshToken
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Auth(Auth::WrongToken))?;

        let token_data = decode::<RefreshToken>(
            bearer.token(),
            &DecodingKey::from_secret(ENV.jwt_refresh_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| Error::Auth(Auth::WrongToken))?;
        let state = AppState::from_ref(state);
        if !check_logged_in_device(&state.pool, &token_data.claims.device_id).await? {
            return Err(Error::Auth(Auth::WrongToken));
        }

        Ok(token_data.claims)
    }
}
