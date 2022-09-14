use crypto_utils::sha::{Algorithm, CryptographicHash};
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: u16,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn passwd_hash(passwd: String) -> String {
        hex::encode(CryptographicHash::hash(
            Algorithm::SHA256,
            passwd.as_bytes(),
        ))
    }
}
