use std::net::SocketAddr;

use warp::Filter;

use crate::error::handle_rejection;

mod error;
mod handlers;
mod payloads;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env file must be present!");

    let hello = routes::hello().and_then(handlers::hello);

    let filter = hello.recover(handle_rejection);

    let bind_address = dotenv::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS should be set in the env!")
        .parse::<SocketAddr>()
        .expect("BIND_ADDRESS should be a valid bind address and port!");

    warp::serve(filter).bind(bind_address).await
}
