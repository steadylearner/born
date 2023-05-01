use born::{
    nested_macro,
    public_enum,
};

pub fn main() {
    public_enum!(
        pub enum WebEventBase {
            PageLoad,
            PageUnload, // , here is required.
        }
    );

    WebEventBase!(); // You have to call it to use.
}