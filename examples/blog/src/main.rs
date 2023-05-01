mod schemas;
use schemas::user::{
    User,
    CreateUser,
};

// Find the latest version and example here https://github.com/tokio-rs/axum
use axum::{
    routing::{post},
    http::StatusCode,
    Json, Router,
};
use std::net::SocketAddr;

// Use these when it is not in examples
// $cargo run 
// $cargo watch -x run 

// Use these when you test
// $cargo run --example blog
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
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
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
