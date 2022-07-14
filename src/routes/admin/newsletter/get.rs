use crate::session_state::TypedSession;
use crate::utils;
use actix_web::HttpResponse;
use actix_web::{self, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn send_newsletters_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(utils::e500)?.is_none() {
        return Ok(utils::see_other("/login"));
    };
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equev="content-type" content="text/html"; charset="utf-8">
    <title>Send a newsletter</title>
</head>
    <body>
        {msg_html}
        <form action="/admin/newsletters" method="post">
            <label>Title
                <input
                    type="test"
                    placeholder="Enter title"
                    name="title"
                >
            </label>
            <br>
            <label>Text content
                <input
                    type="text"
                    placeholder="Enter content"
                    name="text"
                >
            </label>
            <br>
            <label>Html content
                <input
                    type="text"
                    placeholder="Enter content"
                    name="html"
                >
            </label>
            <br>
            <button type="submit">Send</button>
            <br>
            <p><a href="/admin/dashboard">&lt;-Back</a></p>
    </body>
</html>"#,
        )))
}
