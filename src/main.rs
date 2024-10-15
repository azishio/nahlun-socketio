use axum::routing::get;
use axum::Router;
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use std::env;
use tokio::net::TcpListener;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/water_surface", |socket: SocketRef| {
        // TODO Adding authentication
        socket.on("broadcast_request", |socket: SocketRef, Data(message): Data<String>| {
            socket.broadcast().emit("update", &message).ok();
        });
    });

    let addr = env::var("SOCKETIO_HOST").unwrap();

    let listener = TcpListener::bind(addr.clone()).await.unwrap();

    let app = Router::new()
        .route("/", get(|| async move { format!("Hello, Nahlun! by {addr}\nI'm SocketIO Container.\n") }))
        .layer(layer);

    axum::serve(listener, app).await.unwrap();
}
