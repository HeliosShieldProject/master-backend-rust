use rand::Rng;

pub fn shadowsocks() -> String {
    let mut rng = rand::thread_rng();
    let random_numbers: Vec<u8> = (0..32).map(|_| rng.gen_range(0..=255)).collect();
    openssl::base64::encode_block(&random_numbers)
}
