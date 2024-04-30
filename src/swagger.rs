use crate::dto::auth::response::Tokens;
use crate::dto::device::internal::DeviceInfo;
use crate::{
    dto::auth::request::{SignInRequest, SignUpRequest},
    handlers::auth::{sign_in, sign_up},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        sign_in::sign_in, 
        sign_up::sign_up
    ),
    components(
        schemas(
            SignInRequest, 
            SignUpRequest, 
            DeviceInfo,
            Tokens
        ),
    )
)]
pub struct ApiDoc;
