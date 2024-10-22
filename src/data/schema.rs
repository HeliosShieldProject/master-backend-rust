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

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "OAuthProvider"))]
    pub struct OAuthProvider;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ConfigStatus;

    config (id) {
        id -> Uuid,
        private_key -> Text,
        user_ip -> Text,
        server_id -> Uuid,
        status -> ConfigStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Os;
    use super::sql_types::DeviceStatus;

    device (id) {
        id -> Uuid,
        name -> Text,
        os -> Os,
        user_id -> Uuid,
        banned_at -> Nullable<Timestamp>,
        banned_till -> Nullable<Timestamp>,
        status -> DeviceStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Country;

    server (id) {
        id -> Uuid,
        public_key -> Text,
        wireguard_uri -> Text,
        country -> Country,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SessionStatus;

    session (id) {
        id -> Uuid,
        status -> SessionStatus,
        opened_at -> Timestamp,
        closed_at -> Nullable<Timestamp>,
        device_id -> Uuid,
        config_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatus;

    user (id) {
        id -> Uuid,
        email -> Text,
        banned_at -> Nullable<Timestamp>,
        banned_till -> Nullable<Timestamp>,
        status -> UserStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    classic_auth (id) {
        id -> Uuid,
        user_id -> Uuid,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OAuthProvider;

    oauth (id) {
        id -> Uuid,
        user_id -> Uuid,
        provider -> OAuthProvider,
        metadata -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    email_confirmation (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        confirmed -> Bool,
        confirmed_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(classic_auth -> user (user_id));
diesel::joinable!(oauth -> user (user_id));
diesel::joinable!(email_confirmation -> user (user_id));
diesel::joinable!(config -> server (server_id));
diesel::joinable!(device -> user (user_id));
diesel::joinable!(session -> config (config_id));
diesel::joinable!(session -> device (device_id));

diesel::allow_tables_to_appear_in_same_query!(
    config,
    device,
    server,
    session,
    user,
    classic_auth,
    oauth,
    email_confirmation,
);
