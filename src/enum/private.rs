/// Use it to to create, extend and reuse fields from private enum definition.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate born;
/// # fn main() {
/// private_enum!(
///     // pub is required before 'enum' when you use public_enum!
///     enum WebEventBase {
///         PageLoad,
///         PageUnload, // , here is required if you want to extend the fields later.
///     }
/// ); // It is lazy. Nothing is made yet.
///
/// WebEventBase!(); // You have to call it to use the enum.
///
/// fn inspect(event: WebEventBase) {
///     match event {
///         WebEventBase ::PageLoad => println!("page loaded"),
///         WebEventBase ::PageUnload => println!("page unloaded"),
///     }
/// }
///
/// let load    = WebEventBase::PageLoad;
/// let unload  = WebEventBase::PageUnload;
///
/// inspect(load);
/// inspect(unload);
/// # }
/// ```
#[macro_export]
macro_rules! private_enum {
    (enum $commonenum:ident { $( $commonfield:tt )+}) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonenum {
                    () => {
                        enum $commonenum {
                            $( $commonfield )+
                        }
                    };
                    (#[derive($s($arg:tt)+)]) => {
                        #[derive($s($arg)+)]
                        enum $commonenum {
                            $( $commonfield )+
                        }
                    };

                    (enum $name:ident { $s( $field:tt )+}) => {
                        enum $name {
                            $( $commonfield )+
                            $s( $field )+
                        }
                    };
                    (#[derive($s($arg:tt)+)] enum $name:ident { $s( $field:tt )+}) => {
                        #[derive($s($arg)+)]
                        enum $name {
                            $( $commonfield )+
                            $s( $field )+
                        }
                    };

                    (enum $name:ident) => {
                        enum $name {
                            $( $commonfield, )*
                        }
                    };
                    (#[derive($s($arg:tt)+)] enum $name:ident) => {
                        #[derive($s($arg)+)]
                        enum $name {
                            $( $commonfield, )*
                        }
                    };
                }
            };
        }
    };
}
