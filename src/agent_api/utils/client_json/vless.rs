use serde_json::json;
use uuid::Uuid;

pub fn vless(client_id: &Uuid, device_id: &Uuid) -> serde_json::Value {
    json!({
        "id": client_id,
        "flow": "",
        "email": device_id,
        "limitIp": 0,
        "totalGB": 0,
        "expiryTime": 0,
        "enable": true
    })
}
