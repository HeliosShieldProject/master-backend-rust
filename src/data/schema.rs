// @generated automatically by Diesel CLI.

pub mod sql_types {
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
    #[diesel(postgres_type(name = "OAuthProvider"))]
    pub struct OAuthProvider;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "Protocol"))]
    pub struct Protocol;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "SessionStatus"))]
    pub struct SessionStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "UserStatus"))]
    pub struct UserStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::{Os, DeviceStatus};

    device (id) {
        id -> Uuid,
        name -> Text,
        os -> Os,
        user_id -> Uuid,
        status -> DeviceStatus,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::{SessionStatus, Country, Protocol};

    session (id) {
        id -> Uuid,
        device_id -> Uuid,
        protocol -> Protocol,
        country -> Country,
        link -> Text,
        status -> SessionStatus,
        up -> Nullable<BigInt>,
        down -> Nullable<BigInt>,
        opened_at -> Timestamp,
        closed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatus;

    user (id) {
        id -> Uuid,
        email -> Text,
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
diesel::joinable!(device -> user (user_id));
diesel::joinable!(session -> device (device_id));

diesel::allow_tables_to_appear_in_same_query!(
    device,
    session,
    user,
    classic_auth,
    oauth,
    email_confirmation,
);
