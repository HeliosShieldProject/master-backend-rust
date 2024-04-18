use crate::data::schema::sql_types::UserStatus;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub bannedAt: String,
    pub bannedTill: String,
    pub status: UserStatus,
    pub createdAt: String,
    pub updatedAt: String,
}
