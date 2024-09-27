use serde_json::json;
use uuid::Uuid;

use crate::{
    agent_api::{
        dto::{AgentResponse, Client},
        enums::Protocol,
        utils, AgentState,
    },
    data::enums::Country,
    enums::errors::internal::{AgentAPI, Error, Result},
};

pub async fn create_client(
    agent_state: AgentState,
    country: Country,
    host: &str,
    port: u16,
    protocol: Protocol,
    inbound_id: u32,
    device_id: &Uuid,
) -> Result<Client> {
    let cookie = agent_state.get_or_refresh_cookie(&country).await?;
    let agent = agent_state
        .agents
        .get(&country)
        .ok_or(Error::AgentAPI(AgentAPI::Internal))?;
    let client = agent_state.client;

    let (client_body, link) = match protocol {
        Protocol::Vless => {
            let client_id = Uuid::new_v4();
            let client_body = utils::client_json::vless(&client_id, device_id);
            let link = utils::link::vless(&client_id, host, agent.vless_config.port, device_id);

            (client_body, link)
        }
        Protocol::Shadowsocks => {
            let password = utils::password_generator::shadowsocks();
            let client_body = utils::client_json::shadowsocks(&password, device_id);
            let link = utils::link::shadowsocks(
                &agent.password,
                &password,
                host,
                agent.shadowsocks_config.port,
                device_id,
            );

            (client_body, link)
        }
    };

    let res = client
        .post(format!(
            "http://{}:{}/{}/panel/api/inbounds/addClient",
            host, port, agent.secure_path
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
        .json::<AgentResponse>()
        .await?;

    if !res.success {
        return Err(Error::AgentAPI(AgentAPI::Internal));
    }

    Ok(Client { inbound_id, link })
}
