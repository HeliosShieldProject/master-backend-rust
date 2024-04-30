use crate::dto::auth::response::Tokens;
use crate::dto::device::internal::DeviceInfo;
use crate::{dto::auth::request::SignInRequest, handlers::auth::sign_in};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(sign_in::sign_in),
    components(schemas(SignInRequest, DeviceInfo, Tokens),)
)]
pub struct ApiDoc;
