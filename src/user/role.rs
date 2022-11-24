use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
pub enum Role {
    Customer,
    Courier,
    Restaurant,
}
