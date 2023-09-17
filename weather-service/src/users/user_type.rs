use std::io::Write;

use diesel::{sql_types::SqlType, deserialize::{FromSqlRow, FromSql, self}, expression::AsExpression, serialize::{ToSql, Output, self, IsNull}, pg::{Pg, PgValue}};
use serde::{Serialize, Deserialize};

#[derive(SqlType)]
#[diesel(postgres_type(name = "User_Type"))]
pub struct PgUserType;

#[derive(Serialize, Deserialize, Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = PgUserType)]
pub enum UserType {
    Admin,
    User,
}

impl ToSql<PgUserType, Pg> for UserType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
          Self::Admin => out.write_all(b"admin")?,
          Self::User => out.write_all(b"user")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<PgUserType, Pg> for UserType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"admin" => Ok(Self::Admin),
            b"user" => Ok(Self::User),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}