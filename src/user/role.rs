use postgres_types::{FromSql, ToSql};

#[derive(Debug, ToSql, FromSql)]
pub enum Role {
    Customer,
    Courier,
    Restaurant,
}
