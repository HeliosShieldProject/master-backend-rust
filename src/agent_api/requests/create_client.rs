use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    agent_api::{
        dto::{AgentResponse, Client},
        utils, AgentState,
    },
    data::enums::{Country, Protocol},
    enums::errors::internal::{AgentAPI, Error, Result},
};

pub async fn create_client(
    agent_state: &AgentState,
    country: &Country,
    protocol: &Protocol,
    device_id: &Uuid,
) -> Result<Client> {
    let agent_state = agent_state.clone();
    let cookie = agent_state.get_or_refresh_cookie(country).await?;
    let agent = agent_state
        .agents
        .get(country)
        .ok_or(Error::AgentAPI(AgentAPI::Internal))?;
    let client = agent_state.client;

    let (inbound_id, client_body, link) = match protocol {
        Protocol::Vless => {
            let client_id = Uuid::new_v4();
            let client_body = utils::client_json::vless(&client_id, device_id);
            let link =
                utils::link::vless(&client_id, &agent.host, agent.vless_config.port, device_id);
            let inbound_id = agent.vless_config.inbound_id;
            (inbound_id, client_body, link)
        }
        Protocol::Shadowsocks => {
            let password = utils::password_generator::shadowsocks();
            let client_body = utils::client_json::shadowsocks(&password, device_id);
            let link = utils::link::shadowsocks(
                &agent.password,
                &password,
                &agent.host,
                agent.shadowsocks_config.port,
                device_id,
            );
            let inbound_id = agent.shadowsocks_config.inbound_id;

            (inbound_id, client_body, link)
        }
    };

    let res = client
        .post(format!(
            "http://{}:{}/{}/panel/api/inbounds/addClient",
            agent.host, agent.port, agent.secure_path
        ))
        .header("Cookie", cookie)
        .json(&json!({
          "id": inbound_id,
          "settings": {
            "clients": [client_body]
          }
        }
        ))
        .send()
        .await?
        .json::<AgentResponse<Value>>()
        .await?;

    if !res.success {
        return Err(Error::AgentAPI(AgentAPI::Internal));
    }

    Ok(Client { link })
}
