use actix_web::{web, Error, HttpResponse};
use uuid::Uuid;
use crate::lib::general::config::db_config::DbPool;
use crate::lib::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::lib::queuemanagement::logic::queue_management;
use crate::lib::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;


pub async fn find_queues(
    pool: web::Data<DbPool>,
    queue_filters: web::Json<QueueSearchCriteria>
) -> Result<HttpResponse, Error> {
    let search_results =
        queue_management
        ::find_queues(pool, queue_filters.into_inner())
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(search_results))
}

pub async fn get_queue(
    pool: web::Data<DbPool>,
    queue_uid: web::Path<Uuid>
) -> Result<HttpResponse, Error> {

    let uid = queue_uid.into_inner();
    let queue_uid = uid.clone();
    let queue =
        queue_management
        ::find_queue(pool, queue_uid)
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(queue) = queue {
        Ok(HttpResponse::Ok().json(queue))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No queue found with uid: {}", uid));
        Ok(res)
    }

}

pub async fn save_queue(
    pool: web::Data<DbPool>,
    form: web::Json<QueueEto>
) -> Result<HttpResponse, Error> {

    let queue = queue_management::save_queue(pool, form.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(queue))
}

pub async fn delete_queue(
    pool: web::Data<DbPool>,
    queue_uid: web::Path<Uuid>
) -> Result<HttpResponse, Error> {

    queue_management::delete_queue(pool, queue_uid.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}