use std::io::Write;

use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, AsExpression, FromSqlRow, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, ToSchema,
)]
#[diesel(sql_type = crate::data::schema::sql_types::Os)]
#[allow(
    non_camel_case_types,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms
)]
pub enum OS {
    Windows,
    Linux,
    MacOS,
    Android,
    IOS,
    Unknown,
}

impl ToSql<crate::data::schema::sql_types::Os, Pg> for OS {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            OS::Windows => out.write_all(b"Windows")?,
            OS::Linux => out.write_all(b"Linux")?,
            OS::MacOS => out.write_all(b"MacOS")?,
            OS::Android => out.write_all(b"Android")?,
            OS::IOS => out.write_all(b"IOS")?,
            OS::Unknown => out.write_all(b"Unknown")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::data::schema::sql_types::Os, Pg> for OS {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Windows" => Ok(OS::Windows),
            b"Linux" => Ok(OS::Linux),
            b"MacOS" => Ok(OS::MacOS),
            b"Android" => Ok(OS::Android),
            b"IOS" => Ok(OS::IOS),
            b"Unknown" => Ok(OS::Unknown),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
