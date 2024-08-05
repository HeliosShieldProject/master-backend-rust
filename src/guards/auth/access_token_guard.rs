use crate::{
    config::ENV,
    dto::auth::internal::AccessToken,
    enums::errors::external::{AuthError, ExternalError},
    services::device_service::check_logged_in_device,
    state::AppState,
};
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

#[async_trait]
impl<S> FromRequestParts<S> for AccessToken
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ExternalError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ExternalError::AuthError(AuthError::MissingCredentials))?;

        let token_data = decode::<AccessToken>(
            bearer.token(),
            &DecodingKey::from_secret(ENV.jwt_access_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| ExternalError::AuthError(AuthError::WrongToken))?;

        let state = AppState::from_ref(state);
        if !check_logged_in_device(&state.pool, &token_data.claims.device_id).await? {
            return Err(ExternalError::AuthError(AuthError::WrongToken));
        }

        Ok(token_data.claims)
    }
}
