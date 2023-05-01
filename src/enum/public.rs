/// Similar to `private_enum!` but public.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate born;
/// # fn main() {
/// public_enum!(
///     // pub is required before 'enum' when you use public_enum!
///     pub enum WebEventBase {
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
macro_rules! public_enum {
    (
        $(#[$commonattre:meta])*
        pub enum $commonenum:ident { 
            $( $(#[$commonattrf:meta])? $commonfield:tt )+
        }
    ) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonenum {
                    ($s(#[$attre:meta])*) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        pub enum $commonenum {
                            $( $(#[$commonattrf])? $commonfield )+
                        }
                    };

                    (
                        $s(#[$attre:meta])*
                        pub enum $name:ident
                    ) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        enum $name {
                            $( $(#[$commonattrf])? $commonfield, )*
                        }
                    };

                    (
                        $s(#[$attre:meta])*
                        pub enum $name:ident { 
                            $s( $s(#[$attrf:meta])? $field:tt )+
                        }
                    ) => {
                        $(#[$commonattre])*
                        $s(#[$attre])*
                        pub enum $name {
                            $( $(#[$commonattrf])? $commonfield )+
                            $s( $s(#[$attrf])? $field )+
                        }
                    };
                }
            };
        }
    };
}

