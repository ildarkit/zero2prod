use crate::helpers;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = helpers::spawn_app().await;
    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 303);
    helpers::assert_is_redirect_to(&response, "/login");
}