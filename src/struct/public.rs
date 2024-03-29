// https://doc.rust-lang.org/rustdoc/documentation-tests.html#documenting-macros

/// Similar to `private_struct!` but public.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate born;
/// # fn main() {
/// public_struct!(
///     // pub is required before 'struct' when you use public_struct!
///     pub struct MessageBase {
///         pub text: String
///         // text: String // pub is optional in fields.
///     }
/// ); // It is lazy. Nothing is made yet.
///
/// MessageBase!(); // You have to call it to use the struct.
/// let message = MessageBase {
///     text: "First Message".into(),
/// };
/// println!("{}", &message.text);
/// # }
/// ```
#[macro_export]
macro_rules! public_struct {
    (
        $(#[$commonattrs:meta])*
        pub struct $commonstruct:ident {
            $( $(#[$commonattrf:meta])? $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+
            $(,)? 
        }
    ) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonstruct {
                    ($s(#[$attrs:meta])*) => {
                        $(#[$commonattrs])*
                        $s(#[$attrs])*
                        pub struct $commonstruct {
                            $( $(#[$commonattrf])? $commonfieldpub $commonfield: $commonty, )+
                        }
                    };

                    (
                        $s(#[$attrs:meta])*
                        pub struct $name:ident
                    ) => {
                        $(#[$commonattrs])*
                        $s(#[$attrs])*
                        pub struct $name {
                            $( $(#[$commonattrf])? $commonfieldpub $commonfield: $commonty, )+
                        }
                    };

                    (
                        $s(#[$attrs:meta])*
                        pub struct $name:ident { 
                            $s( $s(#[$attrf:meta])? $pub:vis $field:ident: $ty:ty ),+ $s(,)* 
                        }
                    ) => {
                        $(#[$commonattrs])*
                        $s(#[$attrs])*
                        pub struct $name {
                            $( $(#[$commonattrf])? $commonfieldpub $commonfield: $commonty, )+
                            $s( $s(#[$attrf])? $pub $field: $ty ),+
                        }
                    };
                }
            }
        }
    };
}
