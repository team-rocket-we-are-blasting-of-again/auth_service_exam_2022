use std::error::Error;

use bcrypt::{DEFAULT_COST, hash};
use tonic::{Request, Response, Status};

use crate::{
    grpc_auth::{verify_server::Verify, VerifyUserRequest, VerifyUserResponse, user_server::User as GrpcUser, CreateUserRequest, CreateUserResponse},
    jwt::token_manager::TokenManager,
    user::{user_manager::UserManager, user::User, role::Role}
};

pub struct VerifyService {
    token_manager: TokenManager,
}

impl VerifyService {
    pub fn new() -> Self {
        let token_manager = TokenManager::new();
        VerifyService {token_manager}
    }
}

#[tonic::async_trait]
impl Verify for VerifyService {
    async fn verify_user(&self, request: Request<VerifyUserRequest>) -> Result<Response<VerifyUserResponse>, Status> {
        let token = request.into_inner().jwt;
        match self.token_manager.verify_token(token).await {
            Ok(claims) => Ok(Response::new(VerifyUserResponse {
                role_id: claims.custom.id,
                user_role: claims.custom.role.to_string()
            })),
            Err(_) => Err(Status::unauthenticated("Could not verify token"))
        }
    }
}

pub struct UserService {
    user_manager: UserManager
}

impl UserService {
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let user_manager = UserManager::new().await;
        Ok(UserService { user_manager })
    }
}

#[tonic::async_trait]
impl GrpcUser for UserService {
    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<CreateUserResponse>, Status> {
        let inner_request = request.into_inner();
        let role: Role = inner_request.role.try_into().unwrap();
        let password = hash(inner_request.password, DEFAULT_COST).unwrap();
        let user = User {
            id: 0,
            user_role: role,
            role_id: inner_request.role_id,
            email: inner_request.email,
            user_password: password,
        };
        match self.user_manager.create_user(user).await {
            Ok(id) => Ok(Response::new(CreateUserResponse {id})),
            Err(_) => Err(Status::aborted("Failed to create user")) 
        }
    }
}