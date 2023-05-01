// Move this to unit test and more heavier example here later.

use born::{nested_macro, private_struct};

private_struct!(
    struct MessageBase {
        text: String,
    }
);

MessageBase!(); // You have to call it to use.

// $cargo test -- --nocapture
#[test]
fn pass_private_struct() {
    let message = MessageBase {
        text: "First Message".into(),
    };
    println!("{}", &message.text);
}
