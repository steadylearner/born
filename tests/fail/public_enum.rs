use born::{
    nested_macro,
    private_enum,
};

pub fn main() {
    public_enum!(
        enum WebEventBase { // It will fail without pub.
            PageLoad,
            PageUnload,
        }
    );

    WebEventBase!();
}


