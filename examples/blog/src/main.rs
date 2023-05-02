extern crate dotenvy_macro;

use axum::{
    routing::{
        get, 
        // post
    },
    Router,
};
use std::net::SocketAddr;

mod schemas;
mod api;

mod handlers;
use handlers::blog::{
    find_blogs,
    find_blogs_by_title,
    find_blog_by_slug,
};

#[tokio::main]async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route(
            "/api/blogs", get(find_blogs)
        )
        .route(
            "/api/blogs/:title", get(find_blogs_by_title)
        )
        .route(
            "/api/blog/:slug", get(find_blog_by_slug)
        );
        // .route("/api/blog/title/:title", get(find_blog_by_title));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}