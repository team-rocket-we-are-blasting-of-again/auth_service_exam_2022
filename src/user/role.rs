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

impl TryFrom<i32> for Role {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Role::CUSTOMER),
            1 => Ok(Role::COURIER),
            2 => Ok(Role::RESTAURANT),
            _ => Err(())
        }
    }
}