use axum::Json;
use http::StatusCode;
use crate::db::connection::get_db_conn;
use crate::data::task::{PostTaskData,PostTaskResponse};
use crate::db::entities::post_data;
use sea_orm::entity::*;




pub async fn handle_event_post(Json(payload): Json<PostTaskData>) -> Result<Json<PostTaskResponse>, StatusCode> {
    // format!("Received field1: {}, field2: {}", payload.field1, payload.field2)
    let db_conn = get_db_conn().await;

    if payload.address.len() > 0 {
        let post_task = post_data::ActiveModel {
            id: NotSet,
            project: Default::default(),
            event_type: Default::default(),
            address: Default::default(),
            timestamp: Default::default(),
            sign_method: Default::default(),
            sign: Default::default(),
            data: Default::default(),
        };
        post_task.insert(db_conn).await.expect("Fail To Insert Post Data");
        Ok(Json(PostTaskResponse{
            message: "Success".to_string(),
        }))
    }else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}