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
    (pub enum $commonenum:ident { $( $commonfield:tt )+}) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $commonenum {
                    () => {
                        pub enum $commonenum {
                            $( $commonfield )+
                        }
                    };
                    (#[derive($s($arg:tt)+)]) => {
                        #[derive($s($arg)+)]
                        pub enum $commonenum {
                            $( $commonfield )+
                        }
                    };

                    (pub enum $name:ident { $s( $field:tt )+}) => {
                        pub enum $name {
                            $( $commonfield )+
                            $s( $field )+
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub enum $name:ident { $s( $field:tt )+}) => {
                        #[derive($s($arg)+)]
                        pub enum $name {
                            $( $commonfield )+
                            $s( $field )+
                        }
                    };

                    (pub enum $name:ident) => {
                        pub enum $name {
                            $( $commonfield, )*
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub enum $name:ident) => {
                        #[derive($s($arg)+)]
                        pub enum $name {
                            $( $commonfield, )*
                        }
                    };
                }
            }
        }
    };
}

