use crate::{
    schemas::dev_to::DevToBlog, 
    api::blog::BLOGS,
};

use axum::{
    http::StatusCode,
    Json,
    extract::Path,
};

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
// ) -> (StatusCode, Json<Option<DevToBlog>>) {
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
