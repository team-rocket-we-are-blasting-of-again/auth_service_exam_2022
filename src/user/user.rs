use super::role::Role;

pub struct User {
    pub(crate) id: i32,
    pub(crate) user_role: Role,
    pub(crate) role_id: i32,
    pub(crate) email: String,
    pub(crate) user_password: String
}
