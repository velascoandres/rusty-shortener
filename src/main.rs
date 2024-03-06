use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use rusty_shortener::{db::get_pool, link, redirect::redirect_handler, state::AppState};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(
                web::scope("/api").service(
                    web::scope("/link")
                        .service(link::handlers::create_link)
                        .service(link::handlers::get_links),
                ),
            )
            .route("/hey", web::get().to(manual_hello))
            .route("/p/{path}", web::get().to(redirect_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
