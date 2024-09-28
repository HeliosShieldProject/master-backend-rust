use uuid::Uuid;

pub fn vless(client_id: &Uuid, host: &str, port: u16, device_id: &Uuid) -> String {
    format!(
        "vless://{}@{}:{}?type=tcp&security=none#{}",
        client_id, host, port, device_id
    )
}
