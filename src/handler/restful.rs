use axum::Json;
use http::StatusCode;
use crate::db::connection::get_db_conn;
use crate::data::task::{PostTaskData,PostTaskResponse};
use crate::db::entities::post_data;
use sea_orm::entity::*;
use serde_json::value::Value;
use chrono::{DateTime, TimeZone, Utc, Local};
use sea_orm::prelude::DateTimeWithTimeZone;


pub async fn handle_event_post(Json(payload): Json<PostTaskData>) -> Result<Json<PostTaskResponse>, StatusCode> {
    // format!("Received field1: {}, field2: {}", payload.field1, payload.field2)
    let db_conn = get_db_conn().await;

    let json_value: Value = serde_json::to_value(payload.data).unwrap();
    let datetime_utc: DateTime<Utc> = Utc.timestamp(payload.timestamp as i64, 0);
    if payload.address.len() > 0 {
        let post_task = post_data::ActiveModel {
            id: NotSet,
            project: ActiveValue::Set(payload.project),
            event_type: ActiveValue::Set(payload.event_type),
            address: ActiveValue::Set(payload.address),
            timestamp: ActiveValue::Set(DateTimeWithTimeZone::from(datetime_utc)),
            sign_method: ActiveValue::Set(payload.sign_method),
            sign: ActiveValue::Set(payload.sign),
            data: ActiveValue::Set(Option::from(json_value)),
        };
        post_task.insert(db_conn).await.expect("Fail To Insert Post Data");
        Ok(Json(PostTaskResponse{
            message: "Success".to_string(),
        }))
    }else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}