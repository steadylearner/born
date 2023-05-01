use born::{
    nested_macro,
    private_struct,
};

pub fn main() {
    private_struct!(
        struct MessageBase {
            pub text: String // It will fail because of pub.
        }
    );

    MessageBase!();
}

