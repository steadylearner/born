use born::{
    nested_macro,
    private_enum,
};

pub fn main() {
    private_enum!(
        pub enum WebEventBase { // It will fail because of pub.
            PageLoad,
            PageUnload
        }
    );

    WebEventBase!();
}


