/// Use it to to create, extend and reuse fields from private struct definition.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate born;
/// # fn main() {
/// private_struct!(
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
    // (struct $commonstruct:ident { $( $commonfield:ident: $commonty:ty ),+ $(,)? }) => {
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
                    () => {
                        $(#[$commonattrs])*
                        struct $commonstruct {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                        }
                    };

                    (#[derive($s($arg:tt)+)]) => {
                        $(#[$commonattrs])*
                        #[derive($s($arg)+)]
                        struct $commonstruct {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                        }
                    };

                    (
                        #[derive($s($arg:tt)+)] 
                        #[serde($s($args:tt)+)]
                    ) => {
                        $(#[$commonattrs])*
                        #[derive($s($arg)+)]
                        #[serde($s($args)+)]
                        struct $commonstruct {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                        }
                    };

                    (
                        struct $name:ident { 
                            $s( $s(#[$attrf:meta])? $field:ident: $ty:ty ),+ $s(,)* 
                        }
                    ) => {
                        $(#[$commonattrs])*
                        struct $name {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                            $s( $s(#[$attrf])? $field: $ty ),+
                        }
                    };

                    (
                        #[derive($s($arg:tt)+)] 
                        struct $name:ident { 
                            $s( $s(#[$attrf:meta])? $field:ident: $ty:ty ),+ $s(,)* 
                        }
                    ) => {
                        $(#[$commonattrs])*
                        #[derive($s($arg)+)]
                        struct $name {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                            $s( $s(#[$attrf])? $field: $ty ),+
                        }
                    };

                    (
                        #[derive($s($arg:tt)+)] 
                        #[serde($s($args:tt)+)] 
                        struct $name:ident { 
                            $s( $s(#[$attrf:meta])? $field:ident: $ty:ty ),+ $s(,)* 
                        }
                    ) => {
                        $(#[$commonattrs])*
                        #[derive($s($arg)+)]
                        #[serde($s($args)+)]
                        struct $name {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                            $s( $s(#[$attrf])? $field: $ty ),+
                        }
                    };

                    (struct $name:ident) => {
                        $(#[$commonattrs])*
                        pub struct $name {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                        }
                    };
                    
                    (
                        #[derive($s($arg:tt)+)] 
                        struct $name:ident
                    ) => {
                        $(#[$commonattrs])*
                        #[derive($s($arg)+)]
                        struct $name {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                        }
                    };

                    (
                        #[derive($s($arg:tt)+)] 
                        #[serde($s($args:tt)+)] 
                        struct $name:ident
                    ) => {
                        $(#[$commonattrs])*
                        #[derive($s($arg)+)]
                        #[serde($s($args)+)]
                        struct $name {
                            $( $(#[$commonattrf])? $commonfield: $commonty, )+
                        }
                    };

                }
            }
        }
    };
}
