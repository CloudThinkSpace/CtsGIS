use log::info;
use crate::route::root_route;
use tokio::net::TcpListener;
use cts_common::utils::ip::get_local_ip;

pub async fn start() {
    info!("starting server: http://localhost:4000");
    info!("starting server: http://{}:4000", get_local_ip().unwrap());
    let app = root_route();
    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
