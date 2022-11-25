use dotenv::dotenv;
use user::user_manager::UserManager;

use crate::jwt::token_manager::TokenManager;

mod jwt;
mod login_types;
mod rest;
mod user;
mod grpc;

pub mod grpc_auth {
    tonic::include_proto!("auth");
}

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
        .mount(
            "/",
            routes![
                rest::login_customer,
                rest::login_courier,
                rest::login_restaurant
            ],
        )
        .launch()
        .await?;

    Ok(())
}
