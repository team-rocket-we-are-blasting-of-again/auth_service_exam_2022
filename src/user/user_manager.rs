use std::error::Error;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{types::Type, NoTls};

use super::{role::Role, user::User};

pub struct UserManager {
    db_pool: Pool,
}

impl UserManager {
    pub async fn new() -> UserManager {
        let mut db_config = tokio_postgres::Config::new();
        db_config.host(dotenv!("POSTGRES_HOST"));
        db_config.user(dotenv!("POSTGRES_USER"));
        db_config.password(dotenv!("POSTGRES_PASSWORD"));
        db_config.dbname(dotenv!("POSTGRES_DATABASE"));
        let db_manager_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };

        let db_manager = Manager::from_config(db_config, NoTls, db_manager_config);
        let db_pool = Pool::builder(db_manager).build().unwrap();
        let _test_client = db_pool.get().await.unwrap();

        return UserManager { db_pool };
    }

    pub async fn get_from_id(&self, id: i64) -> Result<User, Box<dyn Error + Send + Sync>> {
        let client = self.db_pool.get().await?;
        let statement = client
            .prepare_typed_cached("SELECT * FROM users WHERE id=$1", &[Type::INT4])
            .await?;
        let result = client.query_one(&statement, &[&id]).await?;
        Ok(User {
            id: result.get::<&str, u32>("id"),
            role: result.get::<&str, Role>("role"),
            role_id: result.get::<&str, u32>("role_id"),
            email: result.get::<&str, String>("email"),
            password: result.get::<&str, String>("password"),
            verified: result.get::<&str, bool>("verified"),
        })
    }

    pub async fn get_from_role_and_email(
        &self,
        email: &String,
        role: &Role,
    ) -> Result<User, Box<dyn Error + Send + Sync>> {
        let client = self.db_pool.get().await?;
        let statement = client
            .prepare_typed_cached(
                "SELECT * FROM users WHERE email=$1 AND role=$2;",
                &[Type::TEXT, Type::ANYENUM],
            )
            .await?;
        let result = client.query_one(&statement, &[&email, &role]).await?;
        Ok(User {
            id: result.get::<&str, u32>("id"),
            role: result.get::<&str, Role>("role"),
            role_id: result.get::<&str, u32>("role_id"),
            email: result.get::<&str, String>("email"),
            password: result.get::<&str, String>("password"),
            verified: result.get::<&str, bool>("verified"),
        })
    }
}
