use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RoleClaims {
    pub(crate) role_id: u64,
    pub(crate) id: u64
}