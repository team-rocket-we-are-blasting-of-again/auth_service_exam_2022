use std::error::Error;

use dotenvy::dotenv;
use grpc::{UserService, VerifyService};
use grpc_auth::{verify_server::VerifyServer, user_server::UserServer};
use tokio::task;
use tonic::transport::Server;
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

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    task::spawn(start_grpc_server());
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

async fn start_grpc_server() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = UserService::new().await?;
    let verify_service = VerifyService::new();
    Server::builder()
        .add_service(VerifyServer::new(verify_service))
        .add_service(UserServer::new(user_service))
        .serve(addr)
        .await?;
    Ok(())
}
 