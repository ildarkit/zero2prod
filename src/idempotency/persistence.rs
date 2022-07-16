use super::IdempotencyKey;
use actix_web::HttpResponse;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_saved_response(
    pool: &PgPool,
    idempotency_key: &IdempotencyKey,
    user_id: Uuid,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    let saved_response = sqlx::query!(
        r#"
        SELECT
            response_status_code,
            response_headers,
            response_body
        FROM idempotency
        WHERE
            user_id = $1 AND 
            idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref()
    )
    .fetch_optional(pool)
    .await?;
    todo!()
}
