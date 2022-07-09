use crate::session_state::TypedSession;
use crate::utils;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn change_password_form(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(utils::e500)?.is_none() {
        return Ok(utils::see_other("/login"));
    };
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equev="content-type" content="text/html"; charset="utf-8">
    <title>Change Password</title>
</head>
    <body>
        <form action="/admin/password" method="post">
            <label>Current password
                <input
                    type="password"
                    placeholder="Enter current password"
                    name="current_password"
                >
            </label>
            <br>
            <label>New password
                <input
                    type="password"
                    placeholder="Enter new password"
                    name="new_password"
                >
            </label>
            <br>
            <label>
                <input
                    type="password"
                    placeholder="Type the new password again"
                    name="new_password_check"
                >
            </label>
            <br>
            <p><a href="/admin/dashboard">&lt;-Back</a></p>
    </body>
</html>"#,
    ))
}
