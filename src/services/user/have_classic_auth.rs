use super::get_by_email;

pub async fn have_classic_auth(pool: &deadpool_diesel::postgres::Pool, email: &str) -> bool {
    let user = get_by_email(pool, email).await;
    match user {
        Ok(user) => user.classic_auth.is_some(),
        Err(_) => false,
    }
}