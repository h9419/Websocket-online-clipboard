use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

mod handlers;
mod ws;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type MessageHistory = Arc<Mutex<String>>;
type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let message_history: MessageHistory = Arc::new(Mutex::new(String::from("")));

    println!("Configuring websocket route");
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_message_history(message_history.clone()))
        .and_then(handlers::ws_handler)
        .with(warp::cors().allow_origin("http://localhost"));
    let static_route = warp::path("clipboard")
        .and(warp::fs::dir("/workspace/static"))
        .with(warp::cors().allow_origin("http://localhost"));

    let routes = ws_route.or(static_route);
    println!("Starting server");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
fn with_message_history(
    message_history: MessageHistory,
) -> impl Filter<Extract = (MessageHistory,), Error = Infallible> + Clone {
    warp::any().map(move || message_history.clone())
}
