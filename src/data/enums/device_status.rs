use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Eq, Clone, Copy)]
#[diesel(sql_type = crate::data::schema::sql_types::DeviceStatus)]
pub enum DeviceStatus {
    LoggedIn,
    LoggedOut,
    Revoked,
}

impl ToSql<crate::data::schema::sql_types::DeviceStatus, Pg> for DeviceStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            DeviceStatus::LoggedIn => out.write_all(b"LoggedIn")?,
            DeviceStatus::LoggedOut => out.write_all(b"LoggedOut")?,
            DeviceStatus::Revoked => out.write_all(b"Revoked")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::data::schema::sql_types::DeviceStatus, Pg> for DeviceStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"LoggedIn" => Ok(DeviceStatus::LoggedIn),
            b"LoggedOut" => Ok(DeviceStatus::LoggedOut),
            b"Revoked" => Ok(DeviceStatus::Revoked),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}