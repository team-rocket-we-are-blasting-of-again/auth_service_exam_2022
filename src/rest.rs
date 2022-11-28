use bcrypt::verify;
use rocket::{response::Debug, serde::json::Json, State};
use std::error::Error;

use crate::{
    jwt::token_manager::TokenManager,
    login_types::{LoginRequest, LoginResponse},
    user::{role::Role, user_manager::UserManager},
};

async fn login(
    email: &String,
    password: &String,
    role: Role,
    token_manager: &TokenManager,
    user_manager: &UserManager,
) -> Result<LoginResponse, Debug<Box<dyn Error + Send + Sync>>> {
    let user = user_manager.get_from_role_and_email(email, &role).await?;

    if !verify(password, &user.user_password).unwrap() {
        panic!();
    }

    return Ok(LoginResponse {
        token: token_manager.create_token(user.user_role, user.role_id).await,
    });
}

#[post("/customer/login", data = "<request>")]
pub async fn login_customer(
    request: Json<LoginRequest>,
    token_manager: &State<TokenManager>,
    user_manager: &State<UserManager>,
) -> Result<Json<LoginResponse>, Debug<Box<dyn Error + Send + Sync>>> {
    let response = login(
        &request.email,
        &request.password,
        Role::CUSTOMER,
        &token_manager,
        &user_manager,
    )
    .await?;

    return Ok(Json(response));
}

#[post("/courier/login", data = "<request>")]
pub async fn login_courier(
    request: Json<LoginRequest>,
    token_manager: &State<TokenManager>,
    user_manager: &State<UserManager>,
) -> Result<Json<LoginResponse>, Debug<Box<dyn Error + Send + Sync>>> {
    let response = login(
        &request.email,
        &request.password,
        Role::COURIER,
        &token_manager,
        &user_manager,
    )
    .await?;

    return Ok(Json(response));
}

#[post("/restaurant/login", data = "<request>")]
pub async fn login_restaurant(
    request: Json<LoginRequest>,
    token_manager: &State<TokenManager>,
    user_manager: &State<UserManager>,
) -> Result<Json<LoginResponse>, Debug<Box<dyn Error + Send + Sync>>> {
    let response = login(
        &request.email,
        &request.password,
        Role::RESTAURANT,
        &token_manager,
        &user_manager,
    )
    .await?;

    return Ok(Json(response));
}
