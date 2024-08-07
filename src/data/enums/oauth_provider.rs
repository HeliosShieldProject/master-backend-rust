use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[diesel(sql_type = crate::data::schema::sql_types::OAuthProvider)]
pub enum OAuthProvider {
    Github,
    Google,
    Discord,
}

impl ToSql<crate::data::schema::sql_types::OAuthProvider, Pg> for OAuthProvider {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            OAuthProvider::Github => out.write_all(b"Github")?,
            OAuthProvider::Google => out.write_all(b"Google")?,
            OAuthProvider::Discord => out.write_all(b"Discord")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::data::schema::sql_types::OAuthProvider, Pg> for OAuthProvider {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Github" => Ok(OAuthProvider::Github),
            b"Google" => Ok(OAuthProvider::Google),
            b"Discord" => Ok(OAuthProvider::Discord),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
