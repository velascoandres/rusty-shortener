use actix_web::{web::{self, Redirect}, Responder};

use crate::{errors::CustomError, link, state::AppState};


pub async fn redirect_handler(state: web::Data<AppState>,path: web::Path<(String,)>) -> impl Responder {
    let path = path.into_inner().0;

    let link_result = link::services::fech_by_path(&state.db, &path).await;

    if let Err(error) = link_result {
        return match error {
            CustomError::NotFound(_) => Redirect::to("/not-found").permanent(),
            CustomError::BadRequest(_) => Redirect::to("/not-found").permanent(),
            CustomError::InternalError(_) => Redirect::to("/error").permanent(),
        };
    }

    if let Ok(Some(link)) = link_result {
        return Redirect::to(link.original_link).permanent();
    }

    Redirect::to("/not-found").permanent()

}
