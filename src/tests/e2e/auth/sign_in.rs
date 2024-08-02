#[cfg(test)]
mod test {
    use crate::{
        data::enums::OS,
        dto::{auth::response::Tokens, response},
        enums::errors::external::{AuthError, ExternalError},
        tests::config::ENV,
    };
    use serde_json::json;

    #[tokio::test]
    async fn sign_in_user_not_found() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/sign-in", ENV.master_backend_url);

        let response = client
            .post(&url)
            .json(&json!({
                "email": "sign_in_user_not_found@email.com",
                "password": "1234",
                "device": {
                    "name": "android 1111",
                    "os": OS::Android
                }
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 404);
        let body: response::error::RawResponse = response.json().await.unwrap();
        assert_eq!(
            body.error,
            ExternalError::AuthError(AuthError::UserNotFound).to_string()
        );
    }

    #[tokio::test]
    async fn sign_in() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/sign-in", ENV.master_backend_url);

        let _create_user = client
            .post(&format!("http://{}/auth/sign-up", ENV.master_backend_url))
            .json(&json!({
                "email": "sign_in@email.com",
                "password": "1234",
                "device": {
                    "name": "android 1111",
                    "os": OS::Android
                }
            }))
            .send()
            .await
            .unwrap();

        let response = client
            .post(&url)
            .json(&json!({
                "email": "sign_in@email.com",
                "password": "1234",
                "device": {
                    "name": "android 1111",
                    "os": OS::Android
                }
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body: response::success::RawResponse<Tokens> = response.json().await.unwrap();
        assert!(body.data.is_some());
        let data = body.data.unwrap();
        assert!(!data.access_token.is_empty());
        assert!(!data.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn sign_in_missing_credentials() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/sign-up", ENV.master_backend_url);

        let response = client
            .post(&url)
            .json(&json!({
                "email": "sign_in_missing_credentials@email.com",
                "password": "1234",
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 422);
    }
}
