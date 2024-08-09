use crate::{
    data::{
        models::{ClassicAuth, OAuth, User},
        schema,
    },
    dto::{
        auth::{
            internal::{FullUser, NewUser, OAuthCode, OAuthUser},
            response::Tokens,
        },
        device::internal::{DeviceInfo, NewDevice},
    },
    enums::errors::internal::{AuthError, InternalError},
    services::{device_service, oauth_providers_service},
    state::AppState,
    utils::{hash, token::generate_tokens},
};
use diesel::prelude::*;
use diesel::QueryDsl;
use tracing::{error, info};
use uuid::Uuid;

pub async fn get_by_id(
    pool: &deadpool_diesel::postgres::Pool,
    id: &Uuid,
) -> Result<FullUser, InternalError> {
    let conn = pool.get().await?;
    let id = *id;

    let user = conn
        .interact(move |conn| {
            schema::user::table
                .find(id)
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(|_| InternalError::AuthError(AuthError::UserNotFound))??;

    let oauth: Option<Vec<OAuth>> = conn
        .interact(move |conn| {
            schema::oauth::table
                .filter(schema::oauth::user_id.eq(id))
                .select(OAuth::as_select())
                .load::<OAuth>(conn)
                .optional()
        })
        .await??;

    let classic_auth: Option<ClassicAuth> = conn
        .interact(move |conn| {
            schema::classic_auth::table
                .filter(schema::classic_auth::user_id.eq(id))
                .select(ClassicAuth::as_select())
                .first(conn)
                .optional()
        })
        .await??;

    info!("Got user by id: {}", user.id);

    Ok(FullUser {
        user,
        oauth,
        classic_auth,
    })
}

pub async fn get_by_email(
    pool: &deadpool_diesel::postgres::Pool,
    email: &str,
) -> Result<FullUser, InternalError> {
    let conn = pool.get().await?;
    let email_ = email.to_string();

    let user = conn
        .interact(move |conn| {
            schema::user::table
                .filter(schema::user::email.eq(email_))
                .select(User::as_select())
                .first(conn)
        })
        .await
        .map_err(|_| InternalError::AuthError(AuthError::UserNotFound))??;

    let oauth: Option<Vec<OAuth>> = conn
        .interact(move |conn| {
            schema::oauth::table
                .filter(schema::oauth::user_id.eq(user.id))
                .select(OAuth::as_select())
                .load::<OAuth>(conn)
                .optional()
        })
        .await??;

    let classic_auth: Option<ClassicAuth> = conn
        .interact(move |conn| {
            schema::classic_auth::table
                .filter(schema::classic_auth::user_id.eq(user.id))
                .select(ClassicAuth::as_select())
                .first(conn)
                .optional()
        })
        .await??;

    info!("Got user by email: {}", user.id);

    Ok(FullUser {
        user,
        oauth,
        classic_auth,
    })
}

pub async fn add_user(
    pool: &deadpool_diesel::postgres::Pool,
    email: &str,
) -> Result<FullUser, InternalError> {
    let conn = pool.get().await?;
    let email = email.to_owned();

    if get_by_email(pool, &email).await.is_ok() {
        error!("User already exists: {}", email);
        return Err(InternalError::AuthError(AuthError::UserAlreadyExists));
    }

    let user: User = conn
        .interact(move |conn| {
            diesel::insert_into(schema::user::table)
                .values(schema::user::email.eq(email))
                .get_result(conn)
        })
        .await??;

    info!("Added user: {}", user.id);

    Ok(FullUser {
        user,
        oauth: None,
        classic_auth: None,
    })
}

pub async fn add_classic_auth(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    password: &str,
) -> Result<ClassicAuth, InternalError> {
    let conn = pool.get().await?;
    let user_id = *user_id;
    let hashed_password = hash::hash_password(password).await?;

    let classic_auth: ClassicAuth = conn
        .interact(move |conn| {
            diesel::insert_into(schema::classic_auth::table)
                .values((
                    schema::classic_auth::user_id.eq(user_id),
                    schema::classic_auth::password_hash.eq(hashed_password),
                ))
                .get_result(conn)
        })
        .await??;

    info!("Added classic auth for user: {}", user_id);

    Ok(classic_auth)
}

pub async fn add_oauth(
    state: &AppState,
    oauth_user: &OAuthUser,
    oauth_code: &OAuthCode,
    user_id: &Uuid,
) -> Result<OAuth, InternalError> {
    let conn = state.pool.get().await?;
    let user_id = *user_id;
    let (provider, metadata) = (oauth_code.provider, oauth_user.metadata.clone());

    let current_user = get_by_id(&state.pool, &user_id).await?;

    if current_user.user.email != oauth_user.email {
        error!(
            "OAuth email is different: {} != {}",
            current_user.user.email, oauth_user.email
        );
        return Err(InternalError::AuthError(AuthError::OAuthDifferentEmail));
    }

    if current_user.oauth.is_some()
        && current_user
            .oauth
            .unwrap()
            .iter()
            .any(|oauth| oauth.provider == provider)
    {
        let oauth: OAuth = conn
            .interact(move |conn| {
                diesel::update(schema::oauth::table)
                    .filter(schema::oauth::user_id.eq(user_id))
                    .filter(schema::oauth::provider.eq(provider))
                    .set(schema::oauth::updated_at.eq(diesel::dsl::now))
                    .get_result(conn)
            })
            .await??;
        return Ok(oauth);
    }

    let oauth: OAuth = conn
        .interact(move |conn| {
            diesel::insert_into(schema::oauth::table)
                .values((
                    schema::oauth::user_id.eq(user_id),
                    schema::oauth::provider.eq(provider),
                    schema::oauth::metadata.eq(metadata),
                ))
                .get_result(conn)
        })
        .await??;

    info!("Added OAuth for user: {}", user_id);

    Ok(oauth)
}

pub async fn change_password(
    pool: &deadpool_diesel::postgres::Pool,
    user_id: &Uuid,
    new_password: &str,
) -> Result<User, InternalError> {
    todo!()
}

pub async fn sign_in(
    pool: &deadpool_diesel::postgres::Pool,
    user: &NewUser,
    device: &DeviceInfo,
) -> Result<Tokens, InternalError> {
    let user_db = get_by_email(pool, &user.email).await?;
    if user_db.classic_auth.is_none() {
        error!("User has no classic auth: {}", user.email);
        return Err(InternalError::AuthError(AuthError::NoClassicAuth));
    }
    let classic_auth = user_db.classic_auth.unwrap();

    hash::verify_password(&user.password, &classic_auth.password_hash)
        .await
        .map_err(|_| InternalError::AuthError(AuthError::WrongPassword))?;

    let device = NewDevice {
        name: device.name.clone(),
        os: device.os,
        user_id: user_db.user.id,
    };
    let device = device_service::add_device(pool, &device).await?;

    let tokens = generate_tokens(&user_db.user.id.to_string(), &device.id.to_string()).await?;

    info!("User signed in: {}", user_db.user.id);

    Ok(tokens)
}

pub async fn have_classic_auth(pool: &deadpool_diesel::postgres::Pool, email: &str) -> bool {
    let user = get_by_email(pool, email).await;
    match user {
        Ok(user) => user.classic_auth.is_some(),
        Err(_) => false,
    }
}

pub async fn have_oauth(pool: &deadpool_diesel::postgres::Pool, email: &str) -> bool {
    let user = get_by_email(pool, email).await;
    match user {
        Ok(user) => user.oauth.is_some(),
        Err(_) => false,
    }
}

pub async fn sign_up(
    pool: &deadpool_diesel::postgres::Pool,
    user: &NewUser,
    device: &DeviceInfo,
) -> Result<Tokens, InternalError> {
    if have_classic_auth(pool, &user.email).await {
        error!("User already exists: {}", user.email);
        return Err(InternalError::AuthError(AuthError::UserAlreadyExists));
    }

    let current_user: FullUser = if have_oauth(pool, &user.email).await {
        get_by_email(pool, &user.email).await?
    } else {
        add_user(pool, &user.email).await?
    };

    add_classic_auth(pool, &current_user.user.id, &user.password).await?;

    let device = NewDevice {
        name: device.name.clone(),
        os: device.os,
        user_id: current_user.user.id,
    };

    let device = device_service::add_device(pool, &device).await?;

    let tokens = generate_tokens(&current_user.user.id.to_string(), &device.id.to_string()).await?;

    info!("User signed up user: {}", current_user.user.id);

    Ok(tokens)
}

pub async fn authorize(
    state: &AppState,
    code: &OAuthCode,
    device: &DeviceInfo,
) -> Result<Tokens, InternalError> {
    let oauth_user = oauth_providers_service::authorize_user(state, code).await?;

    let current_user = get_by_email(&state.pool, &oauth_user.email).await;
    let user: FullUser;
    if current_user.is_ok() {
        add_oauth(
            state,
            &oauth_user,
            code,
            &current_user.clone().unwrap().user.id,
        )
        .await?;
        user = current_user.unwrap();
    } else {
        user = add_user(&state.pool, &oauth_user.email).await?;
        add_oauth(state, &oauth_user, code, &user.user.id).await?;
    }

    let device = NewDevice {
        name: device.name.clone(),
        os: device.os,
        user_id: user.user.id,
    };

    let device = device_service::add_device(&state.pool, &device).await?;

    let tokens = generate_tokens(&user.user.id.to_string(), &device.id.to_string()).await?;

    info!("User authorized: {}", user.user.id);

    Ok(tokens)
}
