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
    async fn logout() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/logout", ENV.master_backend_url);

        let user = client
            .post(&format!("http://{}/auth/sign-up", ENV.master_backend_url))
            .json(&json!({
                "email": "logout@email.com",
                "password": "1234",
                "device": {
                    "name": "android 1111",
                    "os": OS::Android
                }
            }))
            .send()
            .await
            .unwrap();

        let user: response::success::RawResponse<Tokens> = user.json().await.unwrap();

        let response = client
            .post(&url)
            .header(
                "Authorization",
                format!("Bearer {}", user.data.unwrap().access_token),
            )
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        let body: response::success::RawResponse<String> = response.json().await.unwrap();
        assert_eq!(body.message, "Logged out successfully");
    }

    #[tokio::test]
    async fn logout_missing_credentials() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/logout", ENV.master_backend_url);

        let response = client.post(&url).send().await.unwrap();

        assert_eq!(response.status(), 400);
        let body: response::error::RawResponse = response.json().await.unwrap();
        assert_eq!(
            body.error,
            ResponseError::AuthError(AuthError::MissingCredentials).to_string()
        );
    }

    #[tokio::test]
    async fn logout_wrong_token() {
        let client = reqwest::Client::new();
        let url = format!("http://{}/auth/logout", ENV.master_backend_url);

        let response = client.post(&url).send().await.unwrap();

        assert_eq!(response.status(), 400);
        let body: response::error::RawResponse = response.json().await.unwrap();
        assert_eq!(
            body.error,
            ResponseError::AuthError(AuthError::MissingCredentials).to_string()
        );
    }
}
