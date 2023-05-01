/// You can make a macro that creates other macros with it.
///
/// ## Example
///
/// ```
/// macro_rules! public_struct {
///     (pub struct $commonstruct:ident { $( $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+ $(,)? }) => {
///         nested_macro! {
///             ($s:tt) => {
///                 macro_rules! $commonstruct {
///                     () => {
///                         pub struct $commonstruct {
///                             $( $commonfieldpub $commonfield: $commonty, )+
///                         }
///                     };
///                 }
///             }
///         }
///     };
/// }
/// ```
#[macro_export]
macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}