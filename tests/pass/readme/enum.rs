use born::{
    nested_macro,
    private_enum,
};

// Define your base private enum here.
private_enum!(
    enum WebEventBase {
        PageLoad,
        PageUnload, // , here is required if you want to extend it.
    }
);

WebEventBase!(
    // #[derive(Debug, Clone, PartialEq)]
    enum WebEvent {
        KeyPress(char),
        Click { x: i64, y: i64 },
        Paste(String),
    }
);

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

// $cargo test -- --nocapture
#[test]
fn pass_readme_enum() {
    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}