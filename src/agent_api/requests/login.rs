use reqwest::header::SET_COOKIE;
use serde_json::Value;
use tracing::error;

use crate::{
    agent_api::{dto::AgentResponse, AgentState},
    data::enums::Country,
    enums::errors::internal::{AgentAPI, Error, Result},
};

pub async fn login(agent_state: &AgentState, country: &Country) -> Result<String> {
    let agent_state = agent_state.clone();
    let client = agent_state.client;
    let agent = agent_state
        .agents
        .get(&country)
        .ok_or(Error::AgentAPI(AgentAPI::Internal))?;
    let params = [("username", &agent.username), ("password", &agent.password)];

    let res = client
        .post(format!(
            "http://{}:{}/{}/login",
            agent.host, agent.port, agent.secure_path
        ))
        .form(&params)
        .send()
        .await?;

    let cookies = res.headers().get_all(SET_COOKIE).iter().last().cloned();
    let body = res.json::<AgentResponse<Value>>().await?;

    if !body.success || cookies.is_none() {
        error!("Login failed: {}", body.msg);
        return Err(Error::AgentAPI(AgentAPI::Internal));
    }

    Ok(cookies.unwrap().to_str().unwrap().to_string())
}
