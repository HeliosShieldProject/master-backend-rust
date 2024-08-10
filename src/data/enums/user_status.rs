use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[diesel(sql_type = crate::data::schema::sql_types::UserStatus)]
pub enum UserStatus {
    Active,
    Banned,
    PermanentlyBanned,
    Deleted,
}

impl ToSql<crate::data::schema::sql_types::UserStatus, Pg> for UserStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            UserStatus::Active => out.write_all(b"Active")?,
            UserStatus::Banned => out.write_all(b"Banned")?,
            UserStatus::PermanentlyBanned => out.write_all(b"PermanentlyBanned")?,
            UserStatus::Deleted => out.write_all(b"Deleted")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::data::schema::sql_types::UserStatus, Pg> for UserStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Active" => Ok(UserStatus::Active),
            b"Banned" => Ok(UserStatus::Banned),
            b"PermanentlyBanned" => Ok(UserStatus::PermanentlyBanned),
            b"Deleted" => Ok(UserStatus::Deleted),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
