extern crate dotenvy_macro;

mod schemas;
use schemas::dev_to::{
    DevToBlog,
};

// Find the latest version and example here https://github.com/tokio-rs/axum
use axum::{
    routing::{
        get, 
        // post
    },
    http::StatusCode,
    Json, Router,
};
use std::net::SocketAddr;

mod api;
use api::blog::{BLOGS};

// Use these when it is not in examples
// $cargo run 
// $cargo watch -x run 

// Use these when you test
// $cargo run --example blog
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/blogs", get(blogs));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// $curl http://localhost:3000/blogs
async fn blogs() -> (StatusCode, Json<Vec<DevToBlog>>) {
    let blogs = BLOGS.get().await; 

    (StatusCode::OK, Json((*blogs).clone()))
}

// $curl -X POST \
//   -H "Content-Type: application/json" \
//   -d '{ "username": "username" }' \
//   http://localhost:3000/users
// async fn create_user(
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     let user = User {
//         id: 1,
//         username: payload.username,
//     };

//     (StatusCode::CREATED, Json(user))
// }
