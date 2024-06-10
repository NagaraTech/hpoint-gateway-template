use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::error::BoxedError;
pub async fn get_ws_conn() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, BoxedError>{
    let ws_url = std::env::var("RELAY_WS_URL").map_err(|e| format!("RELAY_WS_URL not set: {}", e))?;
    let (ws_stream, _) = connect_async(ws_url).await.map_err(|e| format!("Failed to connect to WebSocket: {}", e))?;
    Ok(ws_stream)
}