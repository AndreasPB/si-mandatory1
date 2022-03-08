mod document;
mod routes;

use axum::{extract::Extension, routing::get, Router};
use mongodb::{options::ClientOptions, Client};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let database = client.database("db");
    //    let users = database.collection::<User>("users");
    //
    //    // insert test data
    //    let test_user = User {
    //        name: String::from("Test"),
    //        phone: String::from("1234"),
    //        description: None,
    //    };
    //    users.insert_one(test_user, None).await?;
    //
    //    // print available databases
    //    println!("Databases:");
    //    for db_name in client.list_database_names(None, None).await? {
    //        println!("{}", db_name);
    //    }
    //
    //    // retrieve data from collection
    //    let user: User = users
    //        .find_one(doc! { "name": "Test" }, None)
    //        .await?
    //        .expect("Missing user with name 'Test'");
    //    println!("Retrieved {:?} from database 'db'", user);
    tracing_subscriber::fmt()
        .with_env_filter("tower=trace")
        .pretty()
        .init();
    let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
    let db_client = Client::with_options(client_options)?.database("db");
    let app = Router::new()
        .route("/find-user/:name", get(routes::find_user))
        // add 'db_client' to all request's extensions so handlers can access it.
        .layer(Extension(db_client))
        // logging of requests and responses
        .layer(TraceLayer::new_for_http());

    // run application with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
