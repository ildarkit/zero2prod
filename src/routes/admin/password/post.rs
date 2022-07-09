use crate::authentication::{self, AuthError, Credentials};
use crate::routes::admin::dashboard;
use crate::session_state::TypedSession;
use crate::utils;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = session.get_user_id().map_err(utils::e500)?;
    if user_id.is_none() {
        return Ok(utils::see_other("/login"));
    };
    let user_id = user_id.unwrap();
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new passwords - the field values must match.",
        )
        .send();
        return Ok(utils::see_other("/admin/password"));
    }
    let username = dashboard::get_username(user_id, &pool)
        .await
        .map_err(utils::e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = authentication::validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Ok(utils::see_other("/admin/password"))
            }
            AuthError::UnexpectedError(_) => Err(utils::e500(e).into()),
        };
    }
    todo!()
}
