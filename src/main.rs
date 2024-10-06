use axum::routing::get;
use axum::Router;
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tokio::net::TcpListener;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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
