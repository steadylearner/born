/// Use it to to create, extend and reuse fields from private enum definition.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate born;
/// # fn main() {
/// private_enum!(
///     // pub shouldn't be used before 'enum' when you use private_enum!
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
    (
        // , is required to extend the definition and it is handled by a compiler and Rust rules
        $(#[$commonattre:meta])*
        enum $commonenum:ident { 
            $( $commonfieldlist:tt )+
        }
    ) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonenum {
                    ($s(#[$attre:meta])*) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        enum $commonenum {
                            $( $commonfieldlist )+
                        }
                    };

                    (
                        $s(#[$attre:meta])*
                        enum $name:ident
                    ) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        enum $name {
                            $( $commonfieldlist )+
                        }
                    };

                    (
                        $s(#[$attre:meta])*
                        enum $name:ident { 
                            $s( $fieldlist:tt )+
                        }
                    ) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        enum $name {
                            $( $commonfieldlist )+
                            $s( $fieldlist )+
                        }
                    };
                }
            };
        }
    };
}
