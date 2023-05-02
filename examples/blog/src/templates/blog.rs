use born::{
    public_struct,
    nested_macro,
};

use crate::{
    schemas::dev_to::DevToBlog, 
};

use askama::Template;

public_struct!(
    #[derive(Template)]
    pub struct BlogPostTemplateBase<'a>  {
        pub title: &'a str,
    }
);

BlogPostTemplateBase!(
    #[template(path = "posts.html")]
    pub struct BlogPostsTemplate {
        pub blogs: &'a Vec<DevToBlog>,
    }
);

BlogPostTemplateBase!(
    #[template(path = "post.html")]
    pub struct BlogPostTemplate {
        pub body_markdown: &'a str,
    }
);

// #[derive(Template)]
// #[template(path = "posts.html")]
// pub struct BlogPostsTemplate<'a> {
//     pub title: &'a str,
//     pub blogs: &'a Vec<DevToBlog>,
// }

// // Close the editor when you update some of fields. It is hard to use because the error like this shows up no field `post_title` on type `&BlogPostTemplate<'a>` // This doesn't work well when edited
// #[derive(Template)]
// #[template(path = "post.html")]
// pub struct BlogPostTemplate<'a> {
//     pub title: &'a str,
//     pub body_markdown: &'a str,
// }

// // You can use a custom filter here.
// // mod filters {

// // }

