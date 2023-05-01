use born::{
    nested_macro,
    private_enum,
};

pub fn main() {
    private_enum!(
        enum WebEventBase {
            PageLoad,
            PageUnload,
        }
    );

    WebEventBase!();
}


