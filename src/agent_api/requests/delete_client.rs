use serde_json::Value;
use uuid::Uuid;

use crate::{
    agent_api::{dto::AgentResponse, AgentState},
    data::enums::{Country, Protocol},
    enums::errors::internal::{AgentAPI, Error, Result},
};

pub async fn delete_client(
    agent_state: AgentState,
    country: Country,
    protocol: Protocol,
    device_id: &Uuid,
) -> Result<()> {
    let cookie = agent_state.get_or_refresh_cookie(&country).await?;
    let agent = agent_state
        .agents
        .get(&country)
        .ok_or(Error::AgentAPI(AgentAPI::Internal))?;
    let client = agent_state.client;
    let inbound_id = match protocol {
        Protocol::Vless => agent.vless_config.inbound_id,
        Protocol::Shadowsocks => agent.shadowsocks_config.inbound_id,
    };

    let res = client
        .post(format!(
            "http://{}:{}/{}/panel/api/inbounds/{}/delClient/{}",
            agent.host, agent.port, agent.secure_path, inbound_id, device_id
        ))
        .header("Cookie", cookie)
        .send()
        .await?
        .json::<AgentResponse<Value>>()
        .await?;

    if !res.success {
        return Err(Error::AgentAPI(AgentAPI::Internal));
    }

    Ok(())
}
