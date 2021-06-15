mod dao;
mod models;
mod routes;

use crate::routes::get_routes;
use std::env;

fn get_port() -> u16 {
    match env::var("HELLO_RUST_API_PORT") {
        Ok(variable) => variable.parse::<u16>().unwrap(),
        Err(_) => {
            log::warn!("HELLO_RUST_API_PORT env variable has not been customized!");
            3000
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let port = get_port();
    println!("Server running in port {}", port);
    warp::serve(get_routes()).run(([127, 0, 0, 1], port)).await;
}
