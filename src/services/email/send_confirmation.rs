use diesel::prelude::*;
use resend_rs::types::CreateEmailBaseOptions;
use tracing::info;

use crate::{
    data::{
        models::{EmailConfirmation, User},
        schema,
    },
    enums::errors::internal::Result,
    state::AppState,
};

pub async fn send_confirmation(state: AppState, user: User) -> Result<()> {
    let conn = state.pool.get().await?;
    let user_id = user.id;

    let confirmation = conn
        .interact(move |conn| {
            diesel::insert_into(schema::email_confirmation::table)
                .values(schema::email_confirmation::user_id.eq(user_id))
                .get_result::<EmailConfirmation>(conn)
        })
        .await??;

    let token = confirmation.id.to_string();

    let from = "Helios no-reply <no-reply@helios.michkoff.com>";
    let to = [user.email.clone()];
    let subject = "Confirm your email";

    let email = CreateEmailBaseOptions::new(from, to, subject).with_html(
        format!(
            r#"
            <h1>Confirm your email</h1>
            <p>Click <a href="http://localhost:3000/auth/confirm-email?token={}">here</a> to confirm your email</p>
            "#,
            token
        ).trim()
    );

    state.resend.emails.send(email).await?;

    info!("Sent confirmation email to: {}", user.email);

    Ok(())
}
