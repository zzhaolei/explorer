use anyhow::{Ok, Result};
use explorer_types::database::user::User;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;
pub struct DBConnection {
    pool: SqlitePool,
}
static CONNECTION: OnceCell<DBConnection> = OnceCell::const_new();

impl<'a> DBConnection {
    async fn connect(path: &str) -> Self {
        DBConnection {
            pool: SqlitePool::connect(path)
                .await
                .expect("connect database failed, please check your db file"),
        }
    }
    async fn new(path: &str) -> &'a Self {
        CONNECTION.get_or_init(|| DBConnection::connect(path)).await
    }
    pub fn with<'b, F, R>(&'a self, f: F) -> R
    where
        F: FnOnce(&'a SqlitePool) -> R,
        R: 'b,
        'a: 'b,
    {
        f(&self.pool)
    }
    pub async fn create_user(&self, username: String, password: String) -> Result<User> {
        let password = User::passwd_hash(password);
        let id = self
            .with(|conn| {
                sqlx::query("INSERT INTO `users` VALUES(NULL, ?, ?)")
                    .bind(username)
                    .bind(password)
                    .execute(conn)
            })
            .await?
            .last_insert_rowid();
        let user = self
            .with(|conn| {
                sqlx::query_as::<_, User>("SELECT * FROM `users` WHERE id=?")
                    .bind(id)
                    .fetch_one(conn)
            })
            .await?;

        Ok(user)
    }
    pub async fn get_user(&self, username: String, password: String) -> Result<User> {
        let password = User::passwd_hash(password);
        let user = self
            .with(|conn| {
                sqlx::query_as::<_, User>("SELECT * FROM `users` WHERE username=? AND password=?")
                    .bind(username)
                    .bind(password)
                    .fetch_one(conn)
            })
            .await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {

    use explorer_types::database::user::User;
    use tokio::sync::OnceCell;

    use super::DBConnection;

    static INIT: OnceCell<()> = OnceCell::const_new();

    async fn initialize() {
        INIT.get_or_init(|| async {
            let db = DBConnection::new("sqlite::memory:").await;
            db.with(|conn| sqlx::query(include_str!("../init.sql")).execute(conn))
                .await
                .unwrap();
        })
        .await;
    }

    #[tokio::test]
    async fn test_db_connection() {
        initialize().await;
        let db = DBConnection::new("sqlite::memory:").await;

        let row: (i64,) = db
            .with(|conn| sqlx::query_as("SELECT $1").bind(150_i64).fetch_one(conn))
            .await
            .unwrap();

        assert_eq!(row.0, 150);
    }
    #[tokio::test]
    async fn test_create_user() {
        initialize().await;
        let db = DBConnection::new("sqlite::memory:").await;
        let user = db
            .create_user("username".to_string(), "password".to_string())
            .await
            .unwrap();
        assert_eq!(user.username, "username".to_string());
        assert_eq!(user.password, User::passwd_hash("password".to_string()));

        let user = db
            .get_user("username".to_string(), "password".to_string())
            .await
            .unwrap();
        assert_eq!(user.username, "username".to_string());
    }
}
