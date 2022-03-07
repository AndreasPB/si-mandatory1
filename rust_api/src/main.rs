mod document;
mod routes;
mod state;

use state::SharedState;

#[tokio::main]
async fn main() -> tide::Result<()> {
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
    tide::log::start();
    let state = SharedState::new().await?;
    let mut app = tide::with_state(state);
    app.at("/find-user/:name").get(routes::find_user);
    app.listen("localhost:9000").await?;

    Ok(())
}
