use std::error::Error;

use deadpool_postgres::{Pool, Manager, ManagerConfig, RecyclingMethod};
use tokio_postgres::NoTls;

pub struct UserManager {
    db_pool: Pool
}

impl UserManager {
    pub async fn new() -> UserManager {
        let mut db_config = tokio_postgres::Config::new();
        db_config.host(dotenv!("POSTGRES_HOST"));
        db_config.user(dotenv!("POSTGRES_USER"));
        db_config.password(dotenv!("POSTGRES_PASSWORD"));
        db_config.dbname(dotenv!("POSTGRES_DATABASE"));
        let db_manager_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast
        };

        let db_manager = Manager::from_config(db_config, NoTls, db_manager_config);
        let db_pool = Pool::builder(db_manager).build().unwrap();
        let _test_client = db_pool.get().await.unwrap();

        return UserManager {
            db_pool
        }
    }

    pub async fn get_from_id(&self, id: i64) -> Result<(), Box<dyn Error>>{
        let client = self.db_pool.get().await?;
        
        Ok(())
    }
}