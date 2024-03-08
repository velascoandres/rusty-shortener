use actix_web::{get, post, web::{self, Data}, HttpResponse, Responder};
use validator::Validate;

use crate::{errors::{CustomError, ErrorResponse}, state::AppState};

use super::models::{CreateLink, SearchLink};


#[post("")]
pub async fn create_link(state: Data<AppState>, json: web::Json<CreateLink>) -> impl Responder {
    let is_valid: Result<_, _> = json.validate();


    if is_valid.is_err() {
        return HttpResponse::BadRequest().json(is_valid.err().unwrap());
    }

    let create_result = state.link_repository.create(&json.into_inner()).await;
    
    match create_result {
        Ok(created_link) => HttpResponse::Ok().json(created_link),
        Err(CustomError::BadRequest(error)) => HttpResponse::BadRequest().json(error),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("")]
pub async fn get_links(state: Data<AppState>, query: web::Query<SearchLink>) -> impl Responder {
    let is_valid: Result<_, _> = query.validate();

    if is_valid.is_err() {
        return HttpResponse::BadRequest().json(is_valid.err().unwrap());
    }


    let result = state.link_repository.find_many(query.into_inner()).await;

    match result {
        Ok(fetch_reponse) => HttpResponse::Ok().json(fetch_reponse),
        Err(err) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: err,
        }),
    }
}
