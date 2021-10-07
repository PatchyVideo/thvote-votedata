use actix_web::{HttpRequest, web};

use crate::{context::AppContext, models::{self, ServiceError}};


pub async fn get_all_characters(ctx: web::Data<AppContext>, request: HttpRequest) -> Result<web::Json<models::AllCharacters>, ServiceError> {
    todo!()
}

pub async fn get_all_works(ctx: web::Data<AppContext>, request: HttpRequest) -> Result<web::Json<models::AllWorks>, ServiceError> {
    todo!()
}

pub async fn get_all_musics(ctx: web::Data<AppContext>, request: HttpRequest) -> Result<web::Json<models::AllMusics>, ServiceError> {
    todo!()
}

