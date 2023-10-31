use std::sync::Arc;

use axum::{response::IntoResponse, extract::{State, ws::{WebSocket, WebSocketUpgrade, Message}, Query}, Extension};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Serialize, Deserialize};
use tokio::sync::broadcast::channel;
use uuid::Uuid;

use crate::routes::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct InboxSocketPayload {
    pub inbox: Uuid,
}

pub async fn websocket_handler(
    Extension(user): Extension<Uuid>,
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(req): Query<InboxSocketPayload>
) -> impl IntoResponse {
    let user = user.clone();
    let inbox = req.inbox.clone();
    ws.on_upgrade(move |socket| websocket(user, inbox, socket, state))
}

pub async fn websocket(user: Uuid, inbox: Uuid, stream: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = stream.split();

    if !state.tx_map.contains_key(&inbox) {
        let (tx, _rx) = channel(100);
        state.tx_map.clone().insert(inbox.clone(), tx);
    }

    let tx = state.tx_map.get(&inbox).unwrap();

    let mut rx = tx.subscribe();

    let msg = format!("{} joined.", user.to_string());
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

    if tx.receiver_count() == 0 {
        state.tx_map.clone().remove(&inbox);
    }

}