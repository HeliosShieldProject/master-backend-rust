#[cfg(test)]
mod test {
    use crate::{
        data::enums::OS,
        dto::{auth::response::Tokens, response},
        enums::errors::response::{AuthError, ResponseError},
        tests::config::ENV,
    };
    use serde_json::json;

    #[tokio::test]
    async fn sign_up() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/sign-up", ENV.master_backend_url);

        let response = client
            .post(&url)
            .json(&json!({
                "email": "sign_up@email.com",
                "password": "1234",
                "device": {
                    "name": "android 1111",
                    "os": OS::Android
                }
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 201);
        let body: response::success::RawResponse<Tokens> = response.json().await.unwrap();
        assert!(body.data.is_some());
        let data = body.data.unwrap();
        assert!(!data.access_token.is_empty());
        assert!(!data.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn multiple_sign_up() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/sign-up", ENV.master_backend_url);

        let response = client
            .post(&url)
            .json(&json!({
                "email": "multiple_sign_up@email.com",
                "password": "1234",
                "device": {
                    "name": "android 1111",
                    "os": OS::Android
                }
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 201);
        let body: response::success::RawResponse<Tokens> = response.json().await.unwrap();
        assert!(body.data.is_some());
        let data = body.data.unwrap();
        assert!(!data.access_token.is_empty());
        assert!(!data.refresh_token.is_empty());

        let response = client
            .post(&url)
            .json(&json!({
                "email": "multiple_sign_up@email.com",
                "password": "1234",
                "device": {
                    "name": "android 1111",
                    "os": OS::Android
                }
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 409);
        let body: response::error::RawResponse = response.json().await.unwrap();
        assert_eq!(
            body.error,
            ResponseError::AuthError(AuthError::UserAlreadyExists).to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_missing_credentials() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/sign-up", ENV.master_backend_url);

        let response = client
            .post(&url)
            .json(&json!({
                "email": "sign_up_missing_credentials@email.com",
                "password": "1234",
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 422);
    }
}
