/// Use it to to create, extend and reuse fields from private struct definition.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate born;
/// # fn main() {
/// private_struct!(
///     // pub shouldn't be used before 'strut' when you use private_struct!
///     struct MessageBase {
///         text: String
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
macro_rules! private_struct {
    (
        $(#[$commonattrs:meta])*
        struct $commonstruct:ident {
            $( $(#[$commonattrf:meta])? $commonfield:ident: $commonty:ty ),+
            $(,)? 
        }
    ) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonstruct {
                    ($s(#[$attrs:meta])*) => {
                        $(#[$commonattrs])*
                        $s(#[$attrs])*
                        struct $commonstruct {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                        }
                    };

                    (
                        $s(#[$attrs:meta])*
                        struct $name:ident
                    ) => {
                        $(#[$commonattrs])*
                        $s(#[$attrs])*
                        struct $name {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                        }
                    };

                    (
                        $s(#[$attrs:meta])*
                        struct $name:ident { 
                            $s( $s(#[$attrf:meta])? $field:ident: $ty:ty ),+ $s(,)* 
                        }
                    ) => {
                        $(#[$commonattrs])*
                        $s(#[$attrs])*
                        struct $name {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                            $s( $s(#[$attrf])? $field: $ty ),+
                        }
                    };
                }
            }
        }
    };
}

