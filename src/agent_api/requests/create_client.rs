use serde_json::json;
use uuid::Uuid;

struct Client {
    inbound_id: u32,
    link: String,
}
enum Protocol {
    VLESS,
    Shadowsocks,
}

pub async fn create_client(
    client: reqwest::Client,
    protocol: Protocol,
    url: &str,
    inbound_id: u32,
    device_id: &Uuid,
) {
    match protocol {
        Protocol::VLESS => {
            let res = client
                .post(format!("http://{url}/panel/api/inbounds/addClient"))
                .json(&json!({
                    "id": inbound_id,
                    "settings": {
                      "clients": [
                        {
                          "id": Uuid::new_v4(),
                          "flow": "",
                          "email": device_id,
                          "limitIp": 0,
                          "totalGB": 0,
                          "expiryTime": 0,
                          "enable": true
                        }
                      ]
                    }
                  }
                  ))
                .send()
                .await;
        }
        Protocol::Shadowsocks => {}
    }
}
