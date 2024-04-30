use crate::{
    data::enums::{DeviceStatus, OS},
    dto::{
        auth::{
            request::{ChangePasswordRequest, SignInRequest, SignUpRequest},
            response::Tokens,
        },
        device::{internal::DeviceInfo, response::Device},
        session::{request::CreateSession, response::Session},
    },
    handlers::{
        auth::{change_password, logout, refresh, sign_in, sign_up},
        device::get_devices,
        session::{close_session, create_session},
    },
};
use utoipa::{
    openapi::security::SecurityScheme,
    {Modify, OpenApi},
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "access_token",
            SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
        components.add_security_scheme(
            "refresh_token",
            SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        sign_in::sign_in,
        sign_up::sign_up,
        refresh::refresh,
        logout::logout,
        change_password::change_password,
        create_session::create_session,
        close_session::close_session,
        get_devices::get_devices
    ),
    components(
        schemas(
            SignInRequest,
            SignUpRequest,
            DeviceInfo,
            Device,
            Tokens,
            ChangePasswordRequest,
            Session,
            CreateSession,
            OS,
            DeviceStatus
        )
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;
