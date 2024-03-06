use actix_web::{get, post, web::{self, Data}, HttpResponse, Responder};
use validator::Validate;

use crate::{errors::ErrorResponse, link::services, state::AppState};

use super::models::{CreateLink, SearchLink};


#[post("")]
pub async fn create_link(json: web::Json<CreateLink>) -> impl Responder {
    let is_valid: Result<_, _> = json.validate();
    match is_valid {
        Ok(_) => HttpResponse::Ok().json("success"),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("")]
pub async fn get_links(state: Data<AppState>, query: web::Query<SearchLink>) -> impl Responder {
    let is_valid: Result<_, _> = query.validate();

    if is_valid.is_err() {
        return HttpResponse::BadRequest().json(is_valid.err().unwrap());
    }

    let result = services::fetch_links(&state.db, query.into_inner()).await;

    match result {
        Ok(fetch_reponse) => HttpResponse::Ok().json(fetch_reponse),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: err,
        }),
    }
}