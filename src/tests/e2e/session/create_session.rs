#[cfg(test)]
mod test {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::{Service, ServiceExt};

    use crate::{
        enums::errors::external::{AuthError, ExternalError},
        routers::app_router,
        state::AppState,
    };

    const SIGN_UP: &str = "/auth/sign-up";
    const CREATE_SESSION: &str = "/session";
    const LOGOUT: &str = "/auth/logout";

    #[tokio::test]
    async fn success() {
        let state = AppState::default();
        let mut app = app_router(state.clone()).with_state(state).into_service();

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": "create_session_success@email.com",
                    "password": "1234",
                    "device": {
                        "name": "android 1111",
                        "os": "Android"
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
            .uri(CREATE_SESSION)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", body["data"]["access_token"].as_str().unwrap()),
            )
            .body(Body::from(json!({"country": "UK"}).to_string()))
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

        assert_eq!(body["data"]["server_public_key"], "public_key_1");
        assert_eq!(body["data"]["user_private_key"], "private_key_1");
        assert_eq!(body["data"]["user_ip"], "192.168.1.1");
        assert_eq!(body["data"]["wireguard_uri"], "wireguard_uri_1");
    }

    #[tokio::test]
    async fn wrong_token_try_empty() {
        let state = AppState::default();
        let app = app_router(state.clone()).with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(CREATE_SESSION)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header(http::header::AUTHORIZATION, "Bearer")
                    .body(Body::from(json!({"country": "UK"}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], AuthError::WrongToken.to_string());
    }

    #[tokio::test]
    async fn wrong_token_try_expired() {
        let state = AppState::default();
        let mut app = app_router(state.clone()).with_state(state).into_service();

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": "create_session_expired@email.com",
                    "password": "1234",
                    "device": {
                        "name": "android 1111",
                        "os": "Android"
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
            .uri(LOGOUT)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", body["data"]["access_token"].as_str().unwrap()),
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
            .uri(CREATE_SESSION)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", body["data"]["access_token"].as_str().unwrap()),
            )
            .body(Body::from(json!({"country": "UK"}).to_string()))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], AuthError::WrongToken.to_string());
    }

    #[tokio::test]
    async fn serialization_error() {
        let state = AppState::default();
        let mut app = app_router(state.clone()).with_state(state).into_service();

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": "create_session_serialization_error@email.com",
                    "password": "1234",
                    "device": {
                        "name": "android 1111",
                        "os": "Android"
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
            .uri(CREATE_SESSION)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", body["data"]["access_token"].as_str().unwrap()),
            )
            .body(Body::from(json!({"country": "unknown country"}).to_string()))
            .unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], ExternalError::SerializationError.to_string());
    }
}
