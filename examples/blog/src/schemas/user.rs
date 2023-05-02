// use serde::{Deserialize, Serialize};
// use born::{
//     public_struct,
//     nested_macro,
// };

// public_struct!(
//     pub struct UserBase {
//         pub username: String
//     }
// );

// UserBase!(
//     #[derive(Deserialize)]
//     pub struct CreateUser
// );

// UserBase!(
//     #[derive(Serialize)]
//     pub struct User {
//         pub id: u64,
//     }
// );

// // #[derive(Deserialize)]
// // pub struct CreateUser {
// //     pub username: String,
// // }

// // // the output to our `create_user` handler
// // #[derive(Serialize)]
// // pub struct User {
// //     pub id: u64,
// //     pub username: String,
// // }