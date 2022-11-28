use std::{error::Error, str::FromStr};

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
        db_config.port(dotenv!("POSTGRES_PORT").trim().parse().unwrap());
        let db_manager_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };

        let db_manager = Manager::from_config(db_config, NoTls, db_manager_config);
        let db_pool = Pool::builder(db_manager).build().unwrap();
        let _test_client = db_pool.get().await.unwrap();

        return UserManager { db_pool };
    }

    pub async fn get_from_role_and_email(
        &self,
        email: &String,
        role: &Role,
    ) -> Result<User, Box<dyn Error + Send + Sync>> {
        let client = self.db_pool.get().await?;
        let statement = client
            .prepare_typed_cached(
                "SELECT * FROM users WHERE email=$1 AND user_role=$2;",
                &[Type::TEXT, Type::TEXT],
            )
            .await?;
        let result = client.query_one(&statement, &[&email, &role.to_string()]).await?;
        Ok(User {
            id: result.get::<&str, i32>("id"),
            user_role: Role::from_str(result.get::<&str, &str>("user_role")).unwrap(),
            role_id: result.get::<&str, i32>("role_id"),
            email: result.get::<&str, String>("email"),
            user_password: result.get::<&str, String>("user_password")
        })
    }

    pub async fn create_user(&self, user: User) -> Result<i32, Box<dyn Error + Send + Sync>>{
        let mut client = self.db_pool.get().await?;
        let transaction = client.transaction().await?;
        let statement = transaction
            .prepare_typed_cached("INSERT INTO users (user_role, role_id, email, user_password) VALUES ($1, $2, $3, $4) RETURNING id;",
            &[Type::TEXT, Type::INT4, Type::TEXT, Type::TEXT])
            .await?;
        let role = user.user_role.to_string();
        let result = transaction.query_one(&statement, &[&role, &user.role_id, &user.email, &user.user_password]).await?;
        let id = result.get::<&str, i32>("id");
        transaction.commit().await?;
        Ok(id)
    }
}
