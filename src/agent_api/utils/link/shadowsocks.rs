use uuid::Uuid;

pub fn shadowsocks(
    inbound_password: &str,
    client_password: &str,
    host: &str,
    port: u16,
    device_id: &Uuid,
) -> String {
    let hash = openssl::base64::encode_block(
        format!(
            "2022-blake3-aes-256-gcm:{}:{}",
            inbound_password, client_password
        )
        .as_bytes(),
    );
    format!("ss://{}@{}:{}?type=tcp#{}", hash, host, port, device_id)
}
