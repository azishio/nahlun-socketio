use axum::routing::get;
use axum::Router;
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/tiles", |socket: SocketRef| {
        // TODO Adding authentication
        socket.on("broadcast_request", |socket: SocketRef, Data(message): Data<String>| {
            socket.broadcast().emit("update", &message).ok();
        });
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, Nahlun!" }))
        .layer(layer);

    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
