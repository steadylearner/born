extern crate dotenvy_macro;

use tower_http::services::ServeDir;
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

    render_blog_post_template,
};

#[tokio::main]async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // $curl "http://localhost:3000/static/test.txt"
        // .nest_service("/static", ServeDir::new("static"))
        .nest_service("/static", ServeDir::new("examples/blog/static"))
        .route(
            "/api/blogs", get(find_blogs)
        )
        .route(
            "/api/blogs/:title", get(find_blogs_by_title)
        )
        .route(
            "/api/blog/:slug", get(find_blog_by_slug)
        )
        .route(
            // $curl "http://localhost:3000/blog/how-to-make-a-gui-translator-app-with-python-tkinter-56mi"
            // $curl "http://localhost:3000/blog/<slug>"
            // Or visit at the browser
            "/blog/:slug", get(render_blog_post_template)
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}