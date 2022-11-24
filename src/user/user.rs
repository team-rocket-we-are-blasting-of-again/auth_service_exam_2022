use super::role::Role;

pub struct User {
    pub(crate) id: u32,
    pub(crate) role: Role,
    pub(crate) role_id: u32,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) verified: bool,
}