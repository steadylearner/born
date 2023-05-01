use born::{
    nested_macro,
    private_struct,
};

pub fn main() {
    public_struct!(
        struct MessageBase { // It will fail without pub.
            pub text: String
        }
    );

    MessageBase!();
}

