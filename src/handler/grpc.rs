// use std::collections::HashMap;
// use tonic::{Request, Response, Status};
// use tokio_stream::wrappers::ReceiverStream;
// use tokio::sync::mpsc;
// use crate::handler::gateway::gateway_server::Gateway;
// use crate::handler::gateway::{GatewayResponse,Event};
// use crate::db::entities::{prelude::*, *};
// use sea_orm::*;
// use crate::db::connection::get_db_conn;
//
//
// use sea_orm::entity::*;
// use sea_orm::query::*;
//
//
// use crate::db::entities::{relay_events};
// use tokio::time::{self, Duration};
// use std::sync::{Arc};
// use reqwest::ClientBuilder;
//
//
//
// const EMPTY_NODEID_ERR: &str = "provided node is  was empty";
// const NODE_NOT_EXIST_ERR: &str = "node does not exist";
//
//
// #[derive(Debug)]
// pub struct ToRelayResponse {}
//
// #[tonic::async_trait]
// impl Gateway for ToRelayResponse {
//
//
//     type GetEventStream =ReceiverStream<Result<Event, Status>>;
//
//     async fn get_event(&self, request: Request<()>) -> Result<Response<Self::GetEventStream>, Status> {
//         let conn = get_db_conn().await;
//         let relay_events: Vec<relay_events::Model> = relay_events::Entity::find().all(conn).await.expect("REASON");
//         let (tx, rx) = mpsc::channel(4);
//
//         tokio::spawn(async move {
//             for event in relay_events{
//                 let send_event = Event{
//                     event_type: 0,
//                     timestamp: 0,
//                     address: "".to_string(),
//                     project_name: "".to_string(),
//                     sign: "".to_string(),
//                     sign_method: "".to_string(),
//                     event_date: "".to_string(),
//                     duration: 0,
//                 };
//
//                 if tx.send(Ok(send_event)).await.is_err() {
//                     break;
//                 }
//             }
//
//         });
//
//
//         Ok(Response::new(ReceiverStream::new(rx)))
//     }
//
//
// }
//
