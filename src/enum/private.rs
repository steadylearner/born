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
    (
        $(#[$commonattre:meta])*
        enum $commonenum:ident { 
            $( $(#[$commonattrf:meta])? $commonfield:tt )+
        }
    ) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonenum {
                    ($s(#[$attre:meta])*) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        enum $commonenum {
                            $( $(#[$commonattrf])? $commonfield )+
                        }
                    };

                    (
                        $s(#[$attre:meta])*
                        enum $name:ident
                    ) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        enum $name {
                            $( $(#[$commonattrf])? $commonfield, )*
                        }
                    };

                    (
                        $s(#[$attre:meta])*
                        enum $name:ident { 
                            $s( $s(#[$attrf:meta])? $field:tt )+
                        }
                    ) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        enum $name {
                            $( $(#[$commonattrf])? $commonfield )+
                            $s( $s(#[$attrf])? $field )+
                        }
                    };
                }
            };
        }
    };
}

