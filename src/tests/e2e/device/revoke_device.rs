#[cfg(test)]
mod test {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::{Service, ServiceExt};
    use uuid::Uuid;

    use crate::{
        data::enums::OS,
        enums::errors::external::{Auth, Device},
        routers::app_router,
        state::AppState,
    };

    const SIGN_UP: &str = "/auth/sign-up";
    const SIGN_IN: &str = "/auth/sign-in";
    const GET_DEVICES: &str = "/device";
    const REVOKE_DEVICE: &str = "/device/revoke";

    #[tokio::test]
    async fn success() {
        let state = AppState::default();
        let mut app = app_router(state.clone()).with_state(state).into_service();

        let email = "revoke_device_success@email.com";

        let device1 = "android";
        let os1 = OS::Android;

        let device2 = "iphone";
        let os2 = OS::IOS;

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": email,
                    "password": "1234",
                    "device": {
                        "name": device1,
                        "os": os1
                    }
                })
                .to_string(),
            ))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_object());
        assert!(body["data"]["access_token"].is_string());
        assert!(body["data"]["refresh_token"].is_string());

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_IN)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": email,
                    "password": "1234",
                    "device": {
                        "name": device2,
                        "os": os2
                    }
                })
                .to_string(),
            ))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_object());
        assert!(body["data"]["access_token"].is_string());
        assert!(body["data"]["refresh_token"].is_string());

        let access_token = body["data"]["access_token"].as_str().unwrap();

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(GET_DEVICES)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_array());

        let devices = body["data"].as_array().unwrap();
        assert_eq!(devices.len(), 2);

        let device1 = devices.iter().find(|d| d["name"] == device1).unwrap();

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(format!(
                "{}/{}",
                REVOKE_DEVICE,
                device1["id"].as_str().unwrap()
            ))
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn wrong_token_try_empty() {
        let state = AppState::default();
        let app = app_router(state.clone()).with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(format!("{}/{}", REVOKE_DEVICE, "1"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::AUTHORIZATION, "Bearer")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], Auth::WrongToken.to_string());
    }

    #[tokio::test]
    async fn device_self_revocation() {
        let state = AppState::default();
        let mut app = app_router(state.clone()).with_state(state).into_service();

        let email = "device_self_revocation@email.com";

        let device1 = "android";
        let os1 = OS::Android;

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": email,
                    "password": "1234",
                    "device": {
                        "name": device1,
                        "os": os1
                    }
                })
                .to_string(),
            ))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_object());
        assert!(body["data"]["access_token"].is_string());
        assert!(body["data"]["refresh_token"].is_string());

        let access_token = body["data"]["access_token"].as_str().unwrap();

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(GET_DEVICES)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_array());

        let device = body["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["name"] == device1)
            .unwrap();

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(format!(
                "{}/{}",
                REVOKE_DEVICE,
                device["id"].as_str().unwrap()
            ))
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], Device::SelfRevocation.to_string());
    }

    #[tokio::test]
    async fn device_not_found() {
        let state = AppState::default();
        let mut app = app_router(state.clone()).with_state(state).into_service();

        let email = "revoke_device_not_found@email.com";

        let device1 = "android";
        let os1 = OS::Android;

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": email,
                    "password": "1234",
                    "device": {
                        "name": device1,
                        "os": os1
                    }
                })
                .to_string(),
            ))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_object());
        assert!(body["data"]["access_token"].is_string());
        assert!(body["data"]["refresh_token"].is_string());

        let access_token = body["data"]["access_token"].as_str().unwrap();

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(GET_DEVICES)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_array());

        let device = body["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["name"] == device1)
            .unwrap();

        let mut id: Uuid;
        loop {
            id = Uuid::new_v4();
            if id != Uuid::parse_str(device["id"].as_str().unwrap()).unwrap() {
                break;
            }
        }

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(format!("{}/{}", REVOKE_DEVICE, id))
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], Device::NotFound.to_string());
    }

    #[tokio::test]
    async fn device_already_revoked() {
        let state = AppState::default();
        let mut app = app_router(state.clone()).with_state(state).into_service();

        let email = "device_already_revoked@email.com";

        let device1 = "android";
        let os1 = OS::Android;

        let device2 = "iphone";
        let os2 = OS::IOS;

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": email,
                    "password": "1234",
                    "device": {
                        "name": device1,
                        "os": os1
                    }
                })
                .to_string(),
            ))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_object());
        assert!(body["data"]["access_token"].is_string());
        assert!(body["data"]["refresh_token"].is_string());

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_IN)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": email,
                    "password": "1234",
                    "device": {
                        "name": device2,
                        "os": os2
                    }
                })
                .to_string(),
            ))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_object());
        assert!(body["data"]["access_token"].is_string());
        assert!(body["data"]["refresh_token"].is_string());

        let access_token = body["data"]["access_token"].as_str().unwrap();

        let request = Request::builder()
            .method(http::Method::GET)
            .uri(GET_DEVICES)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_array());

        let devices = body["data"].as_array().unwrap();
        assert_eq!(devices.len(), 2);

        let device1 = devices.iter().find(|d| d["name"] == device1).unwrap();

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(format!(
                "{}/{}",
                REVOKE_DEVICE,
                device1["id"].as_str().unwrap()
            ))
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(format!(
                "{}/{}",
                REVOKE_DEVICE,
                device1["id"].as_str().unwrap()
            ))
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", access_token),
            )
            .body(Body::empty())
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], Device::AlreadyRevoked.to_string());
    }
}
