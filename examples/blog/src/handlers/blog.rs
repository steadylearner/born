use crate::{
    api::blog::BLOGS,
    schemas::dev_to::DevToBlog, 
    templates::blog::{
        BlogPostsTemplate,
        BlogPostTemplate,
    },
};

use axum::{
    http::StatusCode,
    Json,
    extract::Path,
    response::{Html, IntoResponse},
};

// This should be here to this work
use askama::Template;

// $curl http://localhost:3000/api/blogs
pub async fn find_blogs() -> (StatusCode, Json<Vec<DevToBlog>>) {
    let blogs = BLOGS.get().await; 

    (StatusCode::OK, Json((*blogs).clone()))
}

// $curl http://localhost:3000/api/blogs/<title>
// $curl "http://localhost:3000/api/blogs/title/How%to%use%Rust%web%framework%Warp"
// Use this for search at the frontend?
pub async fn find_blogs_by_title(
    Path(title): Path<String>
) -> (StatusCode, Json<Vec<DevToBlog>>) {
    // println!("{:#?}", title);
    let blogs = BLOGS.get().await; 

    let blogs_by_title: Vec<DevToBlog> = blogs
        .into_iter()
        .filter(|blog| blog.title.to_lowercase().contains(&title.to_lowercase().replace("%", " ")))
        .map(|blog| blog.clone())
        .collect();

    (StatusCode::OK, Json(blogs_by_title))
}

// $curl http://localhost:3000/api/blog/<slug>
pub async fn find_blog_by_slug(
    Path(slug): Path<String>
) -> (StatusCode, Json<Option<DevToBlog>>) {
    println!("{:#?}", slug);
    let blogs = BLOGS.get().await; 

    let blog: Vec<&DevToBlog> = blogs
        .into_iter()
        .filter(|blog| blog.slug == slug)
        .collect();

    // println!("{:#?}", blog);

    if let Some(blog_by_slug) = blog.first().cloned() {
        (StatusCode::OK, Json(Some(blog_by_slug.clone())))
    } else {
        (StatusCode::OK, Json(None))
    }
}

// $curl http://localhost:3000/api/blog/<title>
// $curl "http://localhost:3000/api/blog/title/How%to%use%Rust%web%framework%Warp"
// Use this for search?
// async fn find_blog_by_title(
//     Path(title): Path<String>
// ) -> (StatusCode, Json<Option<DevToBlog>>) {
//     println!("{:#?}", title);
//     let blogs = BLOGS.get().await; 

//     let blogs_by_title: Vec<&DevToBlog> = blogs
//         .into_iter()
//         .filter(|blog| blog.title == title.replace("%", " "))
//         .collect();

//     println!("{:#?}", blog);

//     if let Some(blog_by_title) = blog.first().cloned() {
//         (StatusCode::OK, Json(Some(blog_by_title.clone())))
//     } else {
//         (StatusCode::OK, Json(None))
//     }
// }

pub async fn render_blogs() -> impl IntoResponse {
    let blogs = BLOGS.get().await; 

    let template = BlogPostsTemplate{
        title: "Blogs",
        blogs: &blogs,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
    }
}

pub async fn render_blog_post_template(Path(slug): Path<String>) -> impl IntoResponse {
    
    let blogs = BLOGS.get().await; 

    let blog: Vec<&DevToBlog> = blogs
        .into_iter()
        .filter(|blog| blog.slug == slug)
        .collect();

    // println!("{:#?}", blog);

    if let Some(blog_by_slug) = blog.first().cloned() {
        let template = BlogPostTemplate{
            title: &blog_by_slug.title,
            body_markdown: &blog_by_slug.body_markdown,
        };

        match template.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    } else {
        return (StatusCode::NOT_FOUND, "404 not found").into_response();
    }

   
}
