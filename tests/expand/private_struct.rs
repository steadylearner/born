use born::{
    nested_macro,
    private_struct,
};

pub fn main() {
    private_struct!(
        struct MessageBase {
            text: String
            // pub text: String // It will fail.
        }
    );

    MessageBase!();
}

