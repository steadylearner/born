use crate::{
    schemas::dev_to::DevToBlog, 
    api::dev_to::DEV_TO_CLIENT
};

use lazy_static::lazy_static;
use async_once::AsyncOnce;

pub async fn find_blogs() -> Vec<DevToBlog> {
    // https://developers.forem.com/api/v1#tag/articles/operation/getUserPublishedArticles
    let response = DEV_TO_CLIENT.get("https://dev.to/api/articles/me/published")
        .send()
        .await
        .unwrap();
    
    // println!("response {:#?}", &response);

    let blogs: Vec<DevToBlog> = response.json().await.unwrap();
    // println!("{:#?}", &blogs);
    // println!("{:#?}", &blogs.len());

    blogs
}

// You can use this with let blogs = BLOGS.get().await; 
lazy_static! {
    pub static ref BLOGS: AsyncOnce<Vec<DevToBlog>> = AsyncOnce::new(
        async {
            let blogs = find_blogs().await;
            
            blogs
        }
    );
}



