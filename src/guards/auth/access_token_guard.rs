use crate::{
    config::ENV,
    dto::auth::internal::AccessToken,
    enums::errors::response::{AuthError, ResponseError},
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

#[async_trait]
impl<S> FromRequestParts<S> for AccessToken
where
    S: Send + Sync,
{
    type Rejection = ResponseError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ResponseError::AuthError(AuthError::MissingCredentials))?;
        let token_data = decode::<AccessToken>(
            bearer.token(),
            &DecodingKey::from_secret(ENV.jwt_access_secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| ResponseError::AuthError(AuthError::WrongToken))?;

        Ok(token_data.claims)
    }
}
