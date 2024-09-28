use serde_json::json;
use uuid::Uuid;

pub fn shadowsocks(password: &str, device_id: &Uuid) -> serde_json::Value {
    json!({
        "email": device_id,
        "password": password,
        "limitIp": 0,
        "totalGB": 0,
        "expiryTime": 0,
        "enable": true
    })
}
