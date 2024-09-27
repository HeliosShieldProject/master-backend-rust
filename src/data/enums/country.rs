use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Hash)]
#[diesel(sql_type = crate::data::schema::sql_types::Country)]
#[allow(clippy::upper_case_acronyms)]
pub enum Country {
    UK,
    USA,
    Germany,
}

impl ToSql<crate::data::schema::sql_types::Country, Pg> for Country {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Country::UK => out.write_all(b"UK")?,
            Country::USA => out.write_all(b"USA")?,
            Country::Germany => out.write_all(b"Germany")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::data::schema::sql_types::Country, Pg> for Country {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"UK" => Ok(Country::UK),
            b"USA" => Ok(Country::USA),
            b"Germany" => Ok(Country::Germany),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Country::UK => write!(f, "UK"),
            Country::USA => write!(f, "USA"),
            Country::Germany => write!(f, "Germany"),
        }
    }
}
