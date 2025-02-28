use crate::route::root_route;
use tokio::net::TcpListener;

pub async fn start() {
    let app = root_route();
    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
