use axum::{response::IntoResponse, extract::{State, ws::{WebSocket, WebSocketUpgrade, Message}, Path}, Extension};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast::channel;
use uuid::Uuid;

use crate::routes::AppState;

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
            state.tx_map.clone().write().await.insert(inbox.clone(), tx);
        }
    }

    let map = state.tx_map.clone();
    let read_map = map.read().await;
    let tx = read_map.get(&inbox).unwrap();

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
        state.tx_map.clone().write().await.remove(&inbox);
    }

}