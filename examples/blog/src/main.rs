extern crate dotenvy_macro;

mod schemas;
use schemas::user::{
    User,
    CreateUser,
};
use schemas::dev_to::{
    DevToArticle,
};

// Find the latest version and example here https://github.com/tokio-rs/axum
use axum::{
    routing::{post},
    http::StatusCode,
    Json, Router,
};
use std::net::SocketAddr;

mod api;
use api::dev_to::DEV_TO_CLIENT;

// Use these when it is not in examples
// $cargo run 
// $cargo watch -x run 

// Use these when you test
// $cargo run --example blog
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // TODO
    // Extrac this https://dev.to/api to a static variable
    // https://dev.to/api
    let response = DEV_TO_CLIENT.get("https://dev.to/api/articles/me/published")
        .send()
        .await
        .unwrap();

    println!("response {:#?}", &response);

    // Save this to a hashmap and use a route to show them.
    let articles: Vec<DevToArticle> = response.json().await.unwrap();
    println!("{:#?}", &articles);
    println!("{:#?}", &articles.len());

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
