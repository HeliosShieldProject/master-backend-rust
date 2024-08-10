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
        enums::errors::external::{Auth, Error},
        routers::app_router,
        state::AppState,
    };

    const SIGN_UP: &str = "/auth/sign-up";

    #[tokio::test]
    async fn success() {
        let state = AppState::default();
        let app = app_router(state.clone()).with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(SIGN_UP)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({
                            "email": "sign_up@email.com",
                            "password": "1234",
                            "device": {
                                "name": "android 1111",
                                "os": "Android"
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert!(body["data"].is_object());
        assert!(body["data"]["access_token"].is_string());
        assert!(body["data"]["refresh_token"].is_string());
    }

    #[tokio::test]
    async fn user_already_exists() {
        let state = AppState::default();
        let mut app = app_router(state.clone()).with_state(state).into_service();

        let request = Request::builder()
            .method(http::Method::POST)
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": "sign_up_user_already_exists@email.com",
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
            .uri(SIGN_UP)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                json!({
                    "email": "sign_up_user_already_exists@email.com",
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

        assert_eq!(response.status(), StatusCode::CONFLICT);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], Auth::UserAlreadyExists.to_string());
    }

    #[tokio::test]
    async fn serialization_error() {
        let state = AppState::default();
        let app = app_router(state.clone()).with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri(SIGN_UP)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({
                            "email": "sign_up@email.com",
                            "device": {
                                "name": "android 1111",
                                "os": "Android"
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error"], Error::Serialization.to_string());
    }
}
