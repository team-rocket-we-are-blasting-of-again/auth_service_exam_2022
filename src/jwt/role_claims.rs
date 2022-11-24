use serde::{Deserialize, Serialize};

use crate::user::role::Role;

#[derive(Deserialize, Serialize)]
pub struct RoleClaims {
    pub(crate) role: Role,
    pub(crate) id: i32,
}
