use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Eq, Clone, Copy)]
#[diesel(sql_type = crate::data::schema::sql_types::SessionStatus)]
pub enum SessionStatus {
    Active,
    Closed,
}

impl ToSql<crate::data::schema::sql_types::SessionStatus, Pg> for SessionStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SessionStatus::Active => out.write_all(b"Active")?,
            SessionStatus::Closed => out.write_all(b"Closed")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::data::schema::sql_types::SessionStatus, Pg> for SessionStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Active" => Ok(SessionStatus::Active),
            b"Closed" => Ok(SessionStatus::Closed),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
