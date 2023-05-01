use born::{
    nested_macro,
    public_struct,
};

use serde::{Serialize, Deserialize}; 

pub fn main() {
    public_struct!(
        pub struct PostBase {
            pub user_id: i8,
            pub title: String,
            pub body: String,
        }
    );

    PostBase!(
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Post {
            pub id: i8,
        }
    );
}