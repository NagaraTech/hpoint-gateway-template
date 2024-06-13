use axum::{Router, routing::post};
use tower_http::cors::{Any, CorsLayer};
use hpoint_gateway_template::handler::websocket::websocket_event_send;
use hpoint_gateway_template::handler::restful::handle_event_post;
use tokio::time::{self, Duration};
use hpoint_gateway_template::handler::data_process::{process_check_in_events,process_online_time_events};
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;


            process_check_in_events().await.expect("Process Event Data Error");
            process_online_time_events().await.expect("Process Event Data Error");



            websocket_event_send().await.expect("Websocket Sends Event Data Error");
        }
    });


    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers([http::header::AUTHORIZATION]);

    let app = Router::new()
        .nest(
            "/gateway",
            Router::new()
                .route("/post_data", post(handle_event_post))
                .layer(cors),
        );

    let port = std::env::var("RESTFUL_PORT").expect("RESTFUL_PORT not set");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_owned() + &*port).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

}