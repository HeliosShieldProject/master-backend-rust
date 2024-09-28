use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[diesel(sql_type = crate::data::schema::sql_types::Protocol)]
pub enum Protocol {
    Vless,
    Shadowsocks,
}

impl ToSql<crate::data::schema::sql_types::Protocol, Pg> for Protocol {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Protocol::Vless => out.write_all(b"Vless")?,
            Protocol::Shadowsocks => out.write_all(b"Shadowsocks")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::data::schema::sql_types::Protocol, Pg> for Protocol {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Vless" => Ok(Protocol::Vless),
            b"Shadowsocks" => Ok(Protocol::Shadowsocks),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Vless => write!(f, "Vless"),
            Protocol::Shadowsocks => write!(f, "Shadowsocks"),
        }
    }
}
