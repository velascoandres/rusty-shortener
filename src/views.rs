use std::fs;

use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    let html_content = fs::read_to_string("views/index.html").unwrap();

    HttpResponse::Ok().content_type("text/html").body(html_content)
}

pub async fn not_found() -> HttpResponse {
    let html_content = fs::read_to_string("views/not_found.html").unwrap();

    HttpResponse::Ok().content_type("text/html").body(html_content)
}


pub async fn error() -> HttpResponse {
    let html_content = fs::read_to_string("views/error.html").unwrap();

    HttpResponse::Ok().content_type("text/html").body(html_content)
}
