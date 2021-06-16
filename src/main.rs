mod config;
mod dao;
mod models;
mod routes;

use crate::config::CONFIG;
use crate::routes::get_routes;

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Server running in port {}", CONFIG.port);
    warp::serve(get_routes())
        .run(([127, 0, 0, 1], CONFIG.port))
        .await;
}
