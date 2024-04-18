// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ConfigStatus"))]
    pub struct ConfigStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "Country"))]
    pub struct Country;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "DeviceStatus"))]
    pub struct DeviceStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "OS"))]
    pub struct Os;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "SessionStatus"))]
    pub struct SessionStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "UserStatus"))]
    pub struct UserStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConfigStatus;

    Config (id) {
        id -> Text,
        privateKey -> Text,
        userIp -> Text,
        serverId -> Text,
        status -> ConfigStatus,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Os;
    use super::sql_types::DeviceStatus;

    Device (id) {
        id -> Text,
        name -> Text,
        os -> Os,
        userId -> Text,
        bannedAt -> Nullable<Timestamp>,
        bannedTill -> Nullable<Timestamp>,
        revokedAt -> Nullable<Timestamp>,
        status -> DeviceStatus,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Country;

    Server (id) {
        id -> Text,
        publicKey -> Text,
        backendUri -> Text,
        wireguardUri -> Text,
        country -> Country,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SessionStatus;

    Session (id) {
        id -> Text,
        status -> SessionStatus,
        openedAt -> Timestamp,
        closedAt -> Nullable<Timestamp>,
        deviceId -> Text,
        configId -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatus;

    User (id) {
        id -> Text,
        email -> Text,
        password -> Text,
        bannedAt -> Nullable<Timestamp>,
        bannedTill -> Nullable<Timestamp>,
        status -> UserStatus,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::joinable!(Config -> Server (serverId));
diesel::joinable!(Device -> User (userId));
diesel::joinable!(Session -> Config (configId));
diesel::joinable!(Session -> Device (deviceId));

diesel::allow_tables_to_appear_in_same_query!(
    Config,
    Device,
    Server,
    Session,
    User,
);
