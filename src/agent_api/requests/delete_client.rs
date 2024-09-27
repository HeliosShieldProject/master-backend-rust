use uuid::Uuid;

use crate::{
    agent_api::{dto::AgentResponse, AgentState},
    data::enums::Country,
    enums::errors::internal::{AgentAPI, Error, Result},
};

pub async fn delete_client(
    agent_state: AgentState,
    country: Country,
    host: &str,
    port: u16,
    inbound_id: u32,
    device_id: &Uuid,
) -> Result<()> {
    let cookie = agent_state.get_or_refresh_cookie(&country).await?;
    let agent = agent_state
        .agents
        .get(&country)
        .ok_or(Error::AgentAPI(AgentAPI::Internal))?;
    let client = agent_state.client;

    let res = client
        .post(format!(
            "http://{}:{}/{}/panel/api/inbounds/{}/delClient/{}",
            host, port, agent.secure_path, inbound_id, device_id
        ))
        .header("Cookie", cookie)
        .send()
        .await?
        .json::<AgentResponse>()
        .await?;

    if !res.success {
        return Err(Error::AgentAPI(AgentAPI::Internal));
    }

    Ok(())
}
