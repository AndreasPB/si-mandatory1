mod document;
mod routes;

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use mongodb::{options::ClientOptions, Client};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("tower=trace")
        .pretty()
        .init();
    let client_options = ClientOptions::parse("mongodb://root:example@mongo:27017").await?;
    let db_client = Client::with_options(client_options)?.database("db");
    let app = Router::new()
        .route("/user", get(routes::get_all_users))
        .route("/user/:name", get(routes::get_user))
        .route("/user", post(routes::post_user))
        // add 'db_client' to all request's extensions so handlers can access it.
        .layer(Extension(db_client))
        // logging of requests and responses
        .layer(TraceLayer::new_for_http());

    // run application with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
