use core::fmt;
use std::str::FromStr;

use postgres_types::{ToSql, FromSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
pub enum Role {
    CUSTOMER,
    COURIER,
    RESTAURANT,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CUSTOMER" => Ok(Role::CUSTOMER),
            "COURIER" => Ok(Role::COURIER),
            "RESTAURANT" => Ok(Role::RESTAURANT),
            _ => Err(())
        }
    }
}