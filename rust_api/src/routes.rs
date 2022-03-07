use tide::{Body, Request, Response};
use mongodb::{bson::doc};

use crate::document::User;
use crate::state::SharedState;

pub async fn find_user(req: Request<SharedState>) -> tide::Result<impl Into<Response>> {
    let user_name = req.param("name")?;
    let database = req.state().get_db().database("db");
    let users = database.collection::<User>("users");

    let filter = doc! { "name": user_name };
    // TODO: handle the case of a user not existing by
    // returning a nice error message in JSON instead of panicking
    let user: User = users
        .find_one(filter, None)
        .await?
        .expect("Could not find a user with that name");

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&user)?);
    Ok(res)
}
