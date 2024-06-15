use actix_web::{http::header::ContentType, web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

use crate::authentication::UserId;

pub async fn newsletter_form(
    user_id: web::ReqData<UserId>,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let _user_id = user_id.into_inner();

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
            <meta http-equiv="content-type" content="text/html; charset=utf-8">
            <title>Send newsletter</title>
        </head>
        <body>
            {msg_html}
            <form action="/admin/newsletters" method="post">
                <label>Title
                    <input
                        type="text"
                        placeholder="Enter newsletter title"
                        name="title"
                        required
                    >
                </label>
                <br>
                <label>Content
                    <textarea
                        id="newsletter-content"
                        name="content"
                        rows="4"
                        cols="50"
                        required
                    />
                </label>
                <br>
                <button type="submit">Send</button>
            </form>
            <p><a href="/admin/dashboard">&lt;- Back</a></p>
        </body>
        </html>"#,
        )))
}
