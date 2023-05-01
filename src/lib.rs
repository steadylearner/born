// https://doc.rust-lang.org/1.7.0/book/using-rust-without-the-standard-library.html
#![no_std]

#![doc(html_favicon_url = "https://avatars0.githubusercontent.com/u/32325099?s=460&u=cd848fc83d9739939a4ea2d38108c8bcee199109&v=4")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/32325099?s=460&u=cd848fc83d9739939a4ea2d38108c8bcee199109&v=4")]

#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)] // Use it if necessary.

//! [trybuild]: https://github.com/dtolnay/trybuild
//! [macrotest]: https://github.com/eupn/macrotest
//!
//! Reuse(Struct, Enum)
//! =============
//!
//! It provides functional macros to reuse fields from [Struct](https://doc.rust-lang.org/std/keyword.struct.html) and [Enum](https://doc.rust-lang.org/std/keyword.enum.html) definition.
//!
//! ```toml
//! [dependencies]
//! born = "0.0.1"
//! ```
//!
//! <br>
//!
//! ## Examples
//!
//! Here, macros to build public struct and enum are used.
//!
//! If you want to build private struct and enum, just use macros that start with private and shouldn't use `pub` inside.
//!
//! ### Struct
//!
//! Say you build a demo web server to send private messages.
//!
//! ```
//! # #[macro_use] extern crate born;
//! use born::{
//!    nested_macro,
//!    public_struct,
//! };
//!
//! public_struct!(
//!     // pub is required here before struct
//!     pub struct MessageBase {
//!         pub text: String
//!         // pub text: String // , is not required for the struct definition.
//!     }
//! );
//!
//! MessageBase!(
//!     #[derive(Debug, Clone, PartialEq)]
//!     pub struct Message {
//!         pub read: bool,
//!         // read: bool, // pub is optional.
//!     }
//! );
//!
//! impl Message {
//!     fn update_text(&mut self, new_message: String) {
//!         self.text = new_message
//!     }
//!     fn read(&mut self) {
//!         if self.read == false {
//!            self.read = true;
//!        }
//!    }
//! }
//!
//! MessageBase!(
//!    #[derive(Debug, Clone, PartialEq)]
//!    pub struct MessageCreateRequest
//! );
//!
//! MessageBase!(
//!     // #[derive(Debug, Clone, PartialEq)]
//!     pub struct MessageUpdateRequest
//! );
//!
//! fn main() {
//!     let message_create_request = MessageCreateRequest {
//!         text: "I am Steadylearner and 'born' is the crate name.".into(),
//!     };
//!
//!     let mut message = Message {
//!         text: message_create_request.text,
//!         read: false,
//!     };
//!     println!("{:#?}", &message);
//!
//!     assert_eq!(message, message.clone());
//!
//!     let message_update_request = MessageUpdateRequest {
//!         text: "Reuse fields with macros from 'born'.".into(),
//!     };
//!
//!     message.update_text(message_update_request.text);
//!     println!("{:#?}", &message);
//!
//!     message.read();
//!     println!("{:#?}", &message);
//! }
//! ```
//!
//! ### Enum
//!
//! [Compare it with the code example from the Rust documenation.](https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html)
//!
//! ```
//! # #[macro_use] extern crate born;
//! use born::{
//!     nested_macro,
//!     private_enum,
//! };
//!
//! private_enum!(
//!     enum WebEventBase {
//!         PageLoad,
//!         PageUnload, // , here is required if you want to extend it.
//!     }
//! );
//!
//! WebEventBase!(
//!     // #[derive(Debug, Clone, PartialEq)]
//!     enum WebEvent {
//!         KeyPress(char),
//!         Click { x: i64, y: i64 },
//!         Paste(String),
//!     }
//! );
//!
//! fn inspect(event: WebEvent) {
//!     match event {
//!         WebEvent::PageLoad => println!("page loaded"),
//!         WebEvent::PageUnload => println!("page unloaded"),
//!         WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
//!         WebEvent::Paste(s) => println!("pasted \"{}\".", s),
//!         WebEvent::Click { x, y } => {
//!             println!("clicked at x={}, y={}.", x, y);
//!         },
//!     }
//! }
//!
//! fn main() {
//!     let pressed = WebEvent::KeyPress('x');
//!     let pasted  = WebEvent::Paste("my text".to_owned());
//!     let click   = WebEvent::Click { x: 20, y: 80 };
//!     let load    = WebEvent::PageLoad;
//!     let unload  = WebEvent::PageUnload;
//!
//!     inspect(pressed);
//!     inspect(pasted);
//!     inspect(click);
//!     inspect(load);
//!     inspect(unload);
//! }
//! ```
//!
//! ### Rename
//!
//! You can rename struct and enum and reuse the exact same fields.
//!
//! ```
//! # #[macro_use] extern crate born;
//! use born::{
//!     nested_macro,
//!     public_struct,
//! };
//!
//! public_struct!(
//!     pub struct UserBase {
//!         pub first_name: String,
//!         pub last_name: String,
//!         pub email: String,
//!         pub password: String,
//!     }
//! );
//!
//! UserBase!(
//!     pub struct User {
//!         pub id: i64,
//!     }
//! );
//!
//! UserBase!(
//!     pub struct NewUser
//! );
//!
//! // It is equal to write these manually.
//!
//! // pub struct User {
//! //     pub id: i64,
//! //     pub first_name: String,
//! //     pub last_name: String,
//! //     pub email: String,
//! //     pub password: String,
//! // }
//!
//! // pub struct NewUser {
//! //     pub first_name: String,
//! //     pub last_name: String,
//! //     pub email: String,
//! //     pub password: String,
//! // }
//! ```
//!
//! <br>
//!
//! ## Details
//!
//! - Each struct and enum created from the macros are completely unrelevant to each other except they are built(born) from the same definition
//!
//! - When you use `private_struct!` and `private_enum!`, you can't use pub keyword in it and others use them. [It wouldn't be logical if a private struct or private enum can have public fields.](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)
//!
//! - `nested_macro!` is required to use the other macros from this crate. It is used to make a macro that creates other macros.
//! ```rust
//! macro_rules! nested_macro {
//!     ($($body:tt)*) => {
//!         macro_rules! __nested_macro { $($body)+ }
//!         __nested_macro!($);
//!     }
//! }
//! ```
//!
//! <br>
//!
//! ## Why not attribute macro?
//!
//! - [You can reuse the fields with attribute macro also.](https://github.com/steadylearner/born-attribute) But, you need some dependencies.
//!
//! - [If you want more, please read the official documenation about procedural macros.](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)

mod r#enum;
mod r#struct;
mod nested_macro;

// It will only work with $cargo test --doc or $cargo test
#[cfg(doctest)]
doc_comment::doctest!("../README.md");
