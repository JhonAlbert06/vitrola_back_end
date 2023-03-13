use actix_web::{delete, get, post, web, HttpResponse, Responder};

use crate::api_service::Data;

#[get("/Songs")]
async fn get_all_songs(app_data: web::Data<crate::AppStateSong>) -> impl Responder {
    let action = app_data.service_manager_song.api.get_all_songs();
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/Songs/{param}")]
async fn get_song_by(app_data: web::Data<crate::AppStateSong>, param: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager_song.api.get_by_song(&param);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/Songs")]
async fn add_song(app_data: web::Data<crate::AppStateSong>, data: web::Json<Data>) -> impl Responder {
    let action = app_data.service_manager_song.api.create_song(&data);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[post("/Songs/{param}")]
async fn update_song(app_data: web::Data<crate::AppStateSong>, data: web::Json<Data>, param: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager_song.api.update_song(&data, &param);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.modified_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/Songs")]
async fn delete_song(app_data: web::Data<crate::AppStateSong>, data: web::Json<Data>) -> impl Responder {
    let action = app_data.service_manager_song.api.delete_song(&data._id);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.deleted_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


//------------------------------------------------------------------------------------------------------------------
 
#[get("/Songs/List")]
async fn get_all_songs_list(app_data: web::Data<crate::AppStatePlayList>) -> impl Responder {
    let action = app_data.service_manager_play_list.api.get_all_songs_list();
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/Songs/List")]
async fn add_song_list(app_data: web::Data<crate::AppStatePlayList>, data: web::Json<Data>) -> impl Responder {
    let action = app_data.service_manager_play_list.api.create_song_list(&data);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[delete("/Songs/List")]
async fn delete_song_list(app_data: web::Data<crate::AppStatePlayList>, data: web::Json<Data>) -> impl Responder {
    let action = app_data.service_manager_play_list.api.delete_song_list(&data._id);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.deleted_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

//------------------------------------------------------------------------------------------------------------------

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_songs_list);
    cfg.service(add_song_list);
    cfg.service(delete_song_list);
    cfg.service(get_song_by);
    cfg.service(add_song);
    cfg.service(update_song);
    cfg.service(delete_song);
    cfg.service(get_all_songs);
}