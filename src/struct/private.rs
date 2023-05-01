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
    (struct $commonstruct:ident { $( $commonfield:ident: $commonty:ty ),+ $(,)? }) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonstruct {
                    () => {
                        struct $commonstruct {
                            $( $commonfield: $commonty, )+
                        }
                    };

                    (#[derive($s($arg:tt)+)]) => {
                        #[derive($s($arg)+)]
                        struct $commonstruct {
                            $( $commonfield: $commonty, )+
                        }
                    };
                    (#[derive($s($arg:tt)+)] #[serde($s($args:tt)+)]) => {
                        #[derive($s($arg)+)]
                        #[serde($s($args)+)]
                        struct $commonstruct {
                            $( $commonfield: $commonty, )+
                        }
                    };

                    (struct $name:ident { $s( $field:ident: $ty:ty ),+ $s(,)* }) => {
                        struct $name {
                            $( $commonfield: $commonty, )+
                            $s( $field: $ty ),+
                        }
                    };

                    (#[derive($s($arg:tt)+)] struct $name:ident { $s( $field:ident: $ty:ty ),+ $s(,)* }) => {
                        #[derive($s($arg)+)]
                        struct $name {
                            $( $commonfield: $commonty, )+
                            $s( $field: $ty ),+
                        }
                    };
                    (#[derive($s($arg:tt)+)] #[serde($s($args:tt)+)] struct $name:ident { $s( $field:ident: $ty:ty ),+ $s(,)* }) => {
                        #[derive($s($arg)+)]
                        #[serde($s($args)+)]
                        struct $name {
                            $( $commonfield: $commonty, )+
                            $s( $field: $ty ),+
                        }
                    };

                    (struct $name:ident) => {
                        struct $name {
                            $( $commonfield: $commonty, )+
                        }
                    };

                    (#[derive($s($arg:tt)+)] struct $name:ident) => {
                        #[derive($s($arg)+)]
                        struct $name {
                            $( $commonfield: $commonty, )+
                        }
                    };
                    (#[derive($s($arg:tt)+)] #[serde($s($args:tt)+)] struct $name:ident) => {
                        #[derive($s($arg)+)]
                        #[serde($s($args)+)]
                        struct $name {
                            $( $commonfield: $commonty, )+
                        }
                    };
                }
            }
        }
    };
}
