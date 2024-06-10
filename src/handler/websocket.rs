use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use crate::db::connection::get_db_conn;
use crate::db::entities::{relay_events};
use crate::websocket_connection::get_ws_conn;
use sea_orm::entity::*;
use sea_orm::query::*;

use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    InsertEvent(Event),
    QueryEvent,
    Response(Vec<REvent>),
    ResponseWriteId(Id),
}

type Id = i32;

#[derive(Serialize, Deserialize, Debug)]
pub struct REvent {
    pub id: i32,
    pub pk_owner: String,
    pub pk_user: String,
    pub event_meta: Vec<u8>,  // json utf8 ?
    pub event_type: String,
    pub point_amount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    pk_owner: String,
    pk_user: String,
    event_meta: Vec<u8>,
    event_type: String,
}

pub async fn websocket_event_send() -> Result<(), Box<dyn std::error::Error>> {
    match get_ws_conn().await {
        Ok(ws_conn) => {
            println!("Successfully connected to WebSocket");
            let (mut write, mut read) = ws_conn.split();
            let conn = get_db_conn().await;

            let now = Utc::now();
            let current_date = now.date_naive();
            let start_of_day = current_date.and_hms_opt(0, 0, 0);
            let end_of_day = current_date.and_hms_opt(23, 59, 59);

            let relay_events: Vec<relay_events::Model> = relay_events::Entity::find()
                .filter(relay_events::Column::TimeStamp.gte(start_of_day))
                .filter(relay_events::Column::TimeStamp.lte(end_of_day))
                .all(conn).await.expect("REASON");

            for event in relay_events {

                let send_event_bytes: Vec<u8> = bincode::serialize(&crate::handler::gateway::Event {
                    event_type: crate::handler::gateway::EventType::from_str_name(event.event_type.clone()),
                    timestamp: event.time_stamp.timestamp(),
                    address: event.address.clone(),
                    project_name: event.project_name.clone(),
                    sign: event.sign,
                    sign_method: "ED25519".parse().unwrap(),
                    event_date: event.event_date.to_string(),
                    duration: event.duration.unwrap(),
                })?;

                let event_send = Event {
                    pk_owner: event.project_name.clone(),
                    pk_user: event.address.clone(),
                    event_meta: send_event_bytes,
                    event_type: event.event_type.clone().to_string(),
                };

                let message = Message::InsertEvent(event_send);
                let serialized_message = serde_json::to_string(&message)?;

                write.send(WsMessage::Text(serialized_message)).await?;

                while let Some(msg) = read.next().await {
                    let msg = msg?;
                    if let WsMessage::Text(text) = msg {
                        let response: Message = serde_json::from_str(&text)?;
                        match response {
                            Message::Response(events) => {
                                println!("Received events: {:?}", events);
                            }
                            Message::ResponseWriteId(id) => {
                                println!("Received write id : {}", id);
                            }

                            _ => {}
                        }
                    }
                }

            }
        },
        Err(e) => {
            println!("Error connecting to WebSocket: {}", e);
        },
    }

    Ok(())
}
