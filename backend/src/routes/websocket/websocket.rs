use std::sync::Arc;

use axum::{response::IntoResponse, extract::{State, ws::{WebSocket, WebSocketUpgrade, Message}, Path}, Extension};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast::channel;
use uuid::Uuid;

use crate::routes::{AppState, WsChannel};

pub async fn websocket_handler(
    Extension(user): Extension<Uuid>,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> impl IntoResponse {
    println!("->> {:<18} - {}", "HANDLER", "/ws/inbox");

    let user = user.clone();
    let inbox = id.clone();

    ws.on_upgrade(move |socket| websocket(user, inbox, socket, state))
}

pub async fn websocket(user: Uuid, inbox: Uuid, stream: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = stream.split();

    {
        if !state.tx_map.clone().read().await.contains_key(&inbox) {
            let (tx, _rx) = channel(100);
            let ws = WsChannel { sender: Arc::new(tx), user_1: None, user_2: None};
            state.tx_map.clone().write().await.insert(inbox.clone(), ws);
        }

    }

    // Keeps track of the client connections to the socket, with only one connection per user id allowed
    {

        let map = state.tx_map.clone();
        let mut write = map.write().await;
        let ws = write.get(&inbox).unwrap();

        if ws.user_1.is_none() {
            let mut new_ws = write.remove(&inbox).unwrap();
            new_ws.user_1 = Some(user);
            write.insert(inbox, new_ws);
        } else if ws.user_2.is_none() && ws.user_1.unwrap() != user {
            let mut new_ws = write.remove(&inbox).unwrap();
            new_ws.user_2 = Some(user);
            write.insert(inbox, new_ws);
        } else {
            let msg = format!("user {} is already listening on another client.", user);
            let _ = sender.send(Message::Text(msg)).await;
            return;
        }

    }

    let tx = state.tx_map.clone().read().await.get(&inbox).unwrap().sender.clone();

    let mut rx = tx.subscribe();

    let msg = format!("{} joined.", user.to_string());
    let _ = tx.send(msg);

    let msg = format!("{{\"current_users\": \"{}\"}}", tx.receiver_count());
    let _ = tx.send(msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx_clone = tx.clone();
    let name = user.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx_clone.send(format!("{}: {}", name, text));
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let msg = format!("{} left.", user);
    let _ = tx.send(msg);

    let msg = format!("{{\"current_users\": \"{}\"}}", tx.receiver_count());
    let _ = tx.send(msg);

    {

        let map = state.tx_map.clone();
        let mut write = map.write().await;
        let ws = write.get(&inbox).unwrap();

        if ws.user_1.is_some_and(|u| u == user) {
            let mut new_ws = write.remove(&inbox).unwrap();
            new_ws.user_1 = None;
            write.insert(inbox, new_ws);
        } else if ws.user_2.is_some_and(|u| u == user) {
            let mut new_ws = write.remove(&inbox).unwrap();
            new_ws.user_2 = None;
            write.insert(inbox, new_ws);
        }
        
    }

    if tx.receiver_count() == 0 {
        state.tx_map.clone().write().await.remove(&inbox);
    }

}