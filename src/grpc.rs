use tonic::{Request, Response, Status};

use crate::{
    grpc_auth::{verify_server::Verify, VerifyUserRequest, VerifyUserResponse},
    jwt::token_manager::TokenManager,
    user::user_manager::UserManager
};

pub struct GrpcService {
    token_manager: TokenManager,
    user_manager: UserManager
}

#[tonic::async_trait]
impl Verify for GrpcService {
    async fn verify_user(&self, request: Request<VerifyUserRequest>) -> Result<Response<VerifyUserResponse>, Status> {
        let token = request.into_inner().jwt;
        match self.token_manager.verify_token(token).await {
            Ok(claims) => Ok(Response::new(VerifyUserResponse {
                verified: claims.custom.verified,
                user_id: claims.custom.id,
                user_role: claims.role
            })),
            Err(_) => Err(Status::unauthenticated("Could not verify token"))
        }
    }
}