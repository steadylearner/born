use born::{
    nested_macro,
    public_struct,
};

pub fn main() {
    public_struct!(
        pub struct MessageBase {
            text: String
        }
    );

    MessageBase!(); // You have to call it to use.
}