use std::collections::HashMap;

use reqwest::header::SET_COOKIE;
use tracing::error;

use crate::{
    agent_api::dto::AgentResponse,
    enums::errors::internal::{AgentAPI, Error, Result},
};

pub async fn login(
    client: &reqwest::Client,
    url: &str,
    username: &str,
    password: &str,
) -> Result<String> {
    let params = [("username", username), ("password", password)];

    let res = client
        .post(format!("http://{url}/login"))
        .form(&params)
        .send()
        .await?;

    let cookies = res.headers().get_all(SET_COOKIE).iter().last().cloned();
    let body = res.json::<AgentResponse>().await?;

    if !body.success || cookies.is_none() {
        error!("Login failed: {}", body.msg);
        return Err(Error::AgentAPI(AgentAPI::Internal));
    }

    Ok(cookies.unwrap().to_str().unwrap().to_string())
}
