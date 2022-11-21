use deadpool_postgres::{Manager, ManagerConfig, RecyclingMethod, Pool};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use user::user_manager::UserManager;

use crate::jwt::token_manager::TokenManager;

mod rest;
mod jwt;
mod user;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate dotenv_codegen;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let user_manager = UserManager::new().await;
    let _rocket = rocket::build()
        .manage(TokenManager::new())
        .manage(user_manager)
        .mount("/", routes![rest::login])
        .launch()
        .await?;

    Ok(())
}