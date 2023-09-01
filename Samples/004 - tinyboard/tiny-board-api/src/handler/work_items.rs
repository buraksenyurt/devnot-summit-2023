use crate::AppState;
use actix_web::{
    delete, get, post, put, web, web::Json, Error as ActixError, Responder, Result as ActixResult,
    Scope,
};
use sea_orm::DeleteResult;
use serde_json::json;

#[get("")]
async fn get_work_items(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let work_items = state.repository.get_work_items().await;
    Ok(Json(work_items))
}

#[get("/{id}")]
async fn get_work_item_by_id(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> ActixResult<impl Responder, ActixError> {
    let work_item = state.repository.get_work_item_by_id(id.into_inner()).await;
    Ok(Json(work_item))
}

#[post("")]
async fn create_work_item(
    state: web::Data<AppState>,
    payload: Json<crate::repository::work_items::WorkItemRequest>,
) -> ActixResult<impl Responder, ActixError> {
    let work_item = state.repository.create_work_item(payload).await;
    Ok(Json(work_item))
}

#[put("/{id}")]
async fn update_work_item(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    updated: Json<crate::repository::work_items::WorkItemRequest>,
) -> ActixResult<impl Responder, ActixError> {
    let work_item = state
        .repository
        .update_work_item(id.into_inner(), updated)
        .await;
    Ok(Json(work_item))
}

#[delete("/{id}")]
async fn delete_work_item(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> ActixResult<impl Responder, ActixError> {
    let result: DeleteResult = state.repository.delete_work_item(id.into_inner()).await;
    Ok(Json(json!({
        "message":"Work Item silindi",
        "deleted":result.rows_affected
    })))
}

pub fn work_items_handler() -> Scope {
    web::scope("/workitems")
        .service(get_work_items)
        .service(get_work_item_by_id)
        .service(create_work_item)
        .service(update_work_item)
        .service(delete_work_item)
}
