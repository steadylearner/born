mod schemas;
use schemas::user::{
    User,
    CreateUser,
};

use axum::{
    routing::{post},
    http::StatusCode,
    Json, Router,
};
use std::net::SocketAddr;

// TODO
// Test this with cargo exmaple command also?
// Use these when it is not in examples
// $cargo run 
// $cargo watch -x run 

// Use these when you test
// $cargo run --example axum_example
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// $curl -X POST \
//   -H "Content-Type: application/json" \
//   -d '{ "username": "username" }' \
//   http://localhost:3000/users
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// This works
// $curl localhost:3000
// Hello, world!
// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(|| async { "Hello, world!" }));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }