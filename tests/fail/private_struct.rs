use born::{
    nested_macro,
    private_struct,
};

pub fn main() {
    private_struct!(
        pub struct MessageBase { // It will fail because of pub.
            pub text: String // It will fail because of pub.
        }
    );

    MessageBase!();
}

