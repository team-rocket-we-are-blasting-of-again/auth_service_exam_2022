use deadpool_postgres::{Manager, ManagerConfig, RecyclingMethod, Pool};
use dotenv::dotenv;
use tokio_postgres::NoTls;

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
    let _rocket = rocket::build()
        .manage(TokenManager::new())
        .mount("/", routes![rest::login])
        .launch()
        .await?;

    Ok(())
}