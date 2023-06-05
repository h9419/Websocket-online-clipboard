use crate::{ws, Clients, MessageHistory, Result};
use warp::Reply;

pub async fn ws_handler(
    ws: warp::ws::Ws,
    clients: Clients,
    message_history: MessageHistory,
) -> Result<impl Reply> {
    println!("ws_handler");
    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, clients, message_history)))
}
