use serde::Deserialize;
use uuid::Uuid;

use crate::{
    agent_api::{
        dto::{AgentResponse, ClientStats},
        AgentState,
    },
    data::enums::Country,
    enums::errors::internal::{AgentAPI, Error, Result},
};

#[derive(Deserialize, Clone)]
#[allow(non_snake_case)]
struct ClientStatsResponse {
    pub id: u32,
    pub inboundId: u32,
    pub enable: bool,
    pub email: String,
    pub up: i64,
    pub down: i64,
    pub expiryTime: u32,
    pub total: u32,
    pub reset: u32,
}

pub async fn get_client_stats(
    agent_state: &AgentState,
    country: &Country,
    device_id: &Uuid,
) -> Result<ClientStats> {
    let agent_state = agent_state.clone();
    let cookie = agent_state.get_or_refresh_cookie(&country).await?;
    let agent = agent_state
        .agents
        .get(&country)
        .ok_or(Error::AgentAPI(AgentAPI::Internal))?;
    let client = agent_state.client;

    let res = client
        .get(format!(
            "http://{}:{}/{}/panel/api/inbounds/getClientTaffics/{}",
            agent.host, agent.port, agent.secure_path, device_id
        ))
        .header("Cookie", cookie)
        .send()
        .await?
        .json::<AgentResponse<ClientStatsResponse>>()
        .await?;

    if !res.success {
        return Err(Error::AgentAPI(AgentAPI::Internal));
    }

    let stats = ClientStats {
        up: res.obj.clone().unwrap().up,
        down: res.obj.unwrap().down,
    };

    Ok(stats)
}
