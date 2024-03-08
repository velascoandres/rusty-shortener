#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use rusty_shortener::{db::get_pool, link::{self, repository::LinkRepository}, redirect::redirect_handler, state::AppState, views};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection_url = dotenv!("DATABASE_URL");
    let pool = get_pool(connection_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { link_repository: LinkRepository::new(pool.clone()) }))
            .service(
                web::scope("/api").service(
                    web::scope("/link")
                        .service(link::handlers::create_link)
                        .service(link::handlers::get_links),
                ),
            )
            .route("/", web::get().to(views::index))
            .route("/not-found", web::get().to(views::not_found))
            .route("/error", web::get().to(views::error))
            .route("/hey", web::get().to(manual_hello))
            .route("/p/{path}", web::get().to(redirect_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
