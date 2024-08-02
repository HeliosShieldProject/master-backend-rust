use crate::{
    config::ENV,
    dto::auth::internal::RefreshToken,
    enums::errors::external::{AuthError, ExternalError},
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

#[async_trait]
impl<S> FromRequestParts<S> for RefreshToken
where
    S: Send + Sync,
{
    type Rejection = ExternalError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ExternalError::AuthError(AuthError::MissingCredentials))?;
        let token_data = decode::<RefreshToken>(
            bearer.token(),
            &DecodingKey::from_secret(ENV.jwt_refresh_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| ExternalError::AuthError(AuthError::WrongToken))?;

        Ok(token_data.claims)
    }
}
