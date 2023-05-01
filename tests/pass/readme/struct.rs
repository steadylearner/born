use born::{
    nested_macro,
    public_struct,
};

// Define your base public struct here.
public_struct!(
    // pub is required here before struct
    pub struct MessageBase {
        pub text: String
        // pub text: String // , is not required for the struct definition.
    }
);

MessageBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct Message {
        pub read: bool,
        // read: bool, // pub is optional.
    }
);

impl Message {
    fn update_text(&mut self, new_message: String) {
        self.text = new_message
    }
    fn read(&mut self) {
        if self.read == false {
            self.read = true;
        }
    }
}

MessageBase!(
    #[derive(Debug, Clone, PartialEq)]
    pub struct MessageCreateRequest
);

MessageBase!(
    // #[derive(Debug, Clone, PartialEq)]
    pub struct MessageUpdateRequest
);

// $cargo test -- --nocapture
#[test]
fn pass_readme_struct() {
    let message_create_request = MessageCreateRequest {
        text: "I am Steadylearner and 'born' is the crate name.".into(),
    };

    let mut message = Message {
        text: message_create_request.text,
        read: false,
    };
    println!("{:#?}", &message);

    assert_eq!(message, message.clone());

    let message_update_request = MessageUpdateRequest {
        text: "Reuse fields with macros from 'born'.".into(),
    };

    message.update_text(message_update_request.text);
    println!("{:#?}", &message);

    message.read();
    println!("{:#?}", &message);
}