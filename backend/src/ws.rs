use crate::{Client, Clients, MessageHistory};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use serde::{Deserialize, Serialize};
use serde_json::Result;
Jim
#[derive(Serialize, Deserialize)]
struct Action<'a> {
    r#type: &'a str,
    data: Option<String>,
}

pub async fn client_connection(ws: WebSocket, clients: Clients, message_history: MessageHistory) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(
       | result | {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().simple().to_string();

    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        let _ = client_msg(&uuid, msg, &clients, &message_history).await;
    }

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(
    client_id: &str,
    msg: Message,
    clients: &Clients,
    message_history: &MessageHistory,
) -> Result<()> {
    #[cfg(debug_assertions)]
    println!("received message from {}: {:?}", client_id, msg);
    #[cfg(not(debug_assertions))]
    println!("received message from {}", client_id);


    if let Ok(json_data) = msg.to_str() {
        let Action { r#type, data } = serde_json::from_str(json_data)?;
        match r#type {
            "get" => {
                if let Some(client) = clients.lock().await.get(client_id) {
                    if let Some(sender) = &client.sender {
                        // send most recent message to new client
                        println!("sending to client: {}", client_id);
                        let _ = sender.send(Ok(Message::text(message_history.lock().await.clone())));
                    }
                }
            },
            "update" => {
                // save to history
                if let Some(message) = data {
                    *(message_history.lock().await) = message.clone();
                    propagate_msg(Message::text(message), &clients).await;
                }
            },
            "clear" => {
                let message = String::from("");
                *(message_history.lock().await) = message.clone();
                propagate_msg(Message::text(message), &clients).await;
            }
            _ => (),
        }
    }
    Ok(())
}

async fn propagate_msg(
    msg: Message,
    clients: &Clients,
) {
    // propagate it out
    let locked = clients.lock().await.clone();
    for (key, value) in locked.into_iter() {
        if let Some(sender) = &value.sender {
            println!("sending to client: {}", key);
            let _ = sender.send(Ok(msg.clone()));
        }
    }
}