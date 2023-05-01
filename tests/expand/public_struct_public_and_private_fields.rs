use born::{
    nested_macro,
    public_struct,
};

pub fn main() {
    public_struct!(
        pub struct MessageBase {
            pub text: String,
            read: bool,
        }
    );

    MessageBase!(); // You have to call it to use.
}