use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Eq, Clone, Copy)]
#[diesel(sql_type = crate::data::schema::sql_types::ConfigStatus)]
pub enum ConfigStatus {
    InUse,
    NotInUse,
}

impl ToSql<crate::data::schema::sql_types::ConfigStatus, Pg> for ConfigStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ConfigStatus::InUse => out.write_all(b"InUse")?,
            ConfigStatus::NotInUse => out.write_all(b"NotInUse")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::data::schema::sql_types::ConfigStatus, Pg> for ConfigStatus {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"InUse" => Ok(ConfigStatus::InUse),
            b"NotInUse" => Ok(ConfigStatus::NotInUse),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
