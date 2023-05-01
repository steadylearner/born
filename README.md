[trybuild]: https://github.com/dtolnay/trybuild
[macrotest]: https://github.com/eupn/macrotest

Reuse common parts of Struct and Enum
=============

<!-- Include donate button if people find it useful later. -->

<!-- [![Build Status](https://travis-ci.org/steadylearner/born.svg?branch=master)](https://travis-ci.org/steadylearner/born) -->
[![Image Crate](https://img.shields.io/crates/v/born.svg)](https://crates.io/crates/born)
[![Image Doc](https://img.shields.io/badge/rust-documentation-blue.svg)](https://docs.rs/born/
)

It provides functional macros to reuse fields from [Struct](https://doc.rust-lang.org/std/keyword.struct.html) and [Enum](https://doc.rust-lang.org/std/keyword.enum.html) definition.

```toml
[dependencies]
born = { git = "https://github.com/steadylearner/born", branch = "master" }
```

[![born crate example](https://raw.githubusercontent.com/steadylearner/born/master/unite_rust_and_python_with_born.png)](https://raw.githubusercontent.com/steadylearner/born/master/unite_rust_and_python_with_born.png)

<br>

## Why this library?

You can define common fields in Rust struct and enum once and reuse them to remove code duplication. Use it when you want to reuse the same fields for the structs like the example below.

```rust
use born::{
    nested_macro,
    public_struct,
};

public_struct!(
    pub struct UserBase {
        username: String,
        email: String,
        full_name: Option<String>,
    }
);

UserBase!(
    pub struct UserIn {
        pub password: String,
    }
);

// Reuse with the same fields.
UserBase!(
    pub struct UserOut
);

UserBase!(
    pub struct UserInDB {
        pub hashed_password: String,
    }
);
```

Compare it with Python code below from [FAST API](https://fastapi.tiangolo.com/tutorial/extra-models/#reduce-duplication) that inspired this library.

```py
from pydantic import BaseModel, EmailStr

class UserBase(BaseModel):
    username: str
    email: EmailStr
    full_name: str = None


class UserIn(UserBase):
    password: str

# Reuse with the same fields.
class UserOut(UserBase):
    pass


class UserInDB(UserBase):
    hashed_password: str
```

You can see almost same thing is done here to remove code duplication in both parts.

But, different from Python, there is no inheritance of fields with macros from **born**. It is lazily built(born) by your first struct or enum definition.

Everything made from them are completely irrelevant to each other except they share the same definition. There is no memory share or something like that.

The macros from this library are **lazy struct and enum builders** to remove code duplication. It is possible with the power of the Rust macro.

## Examples

Here, macros to build public struct and enum are used.

If you want to build private struct and enum, just use macros that start with private and shouldn't use `pub` inside.

### Struct

Say you build a simple demo web server to send private messages.

```rust
use born::{
    nested_macro,
    public_struct,
};

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

fn main() {
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
```

You can also use the public_struct! and private_struct! with serde derive. For example, you can rename fields to camelCase with `#[serde(rename_all = "camelCase")]` etc.

```rust
// Cargo.toml
// born = { git = "https://github.com/steadylearner/born", branch = "master" }

use born::{
    nested_macro,
    public_struct,
};

use serde::{Serialize, Deserialize}; 

public_struct!(
    pub struct PostBase {
        pub user_id: i8,
        pub title: String,
        pub body: String,
    }
);

PostBase!(
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Post {
        pub id: i8,
    }
);
```

### Enum

[Compare it with the code example from the Rust documenation.](https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html)

```rust
use born::{
    nested_macro,
    private_enum,
};

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

fn main() {
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
```

<br>

## Details

- Each struct and enum created from the macros are completely unrelevant to each other except they are built(born) from the same definition.

- When you use `private_struct!` and `private_enum!`, you can't use pub keyword in it and others use them. [It wouldn't be logical if a private struct or private enum can have public fields.](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)

- `nested_macro!` is required to use the other macros from this crate. It is used to make a macro that creates other macros.

```rust
macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}
```

- You can also use attributes for the common parts.

```rust
// Cargo.toml
// born = { git = "https://github.com/steadylearner/born", branch = "master" }

use born::{nested_macro, public_struct};
use serde::{Serialize, Deserialize}; 

public_struct!(
    #[derive(Deserialize, Serialize, Debug)]
    pub struct PersonBase {
        #[serde(rename = "person_email")]
        email: String,
    }
);

PersonBase!(pub struct Person); // You have to call it to use.
PersonBase!(pub struct PersonWithName {
    #[serde(rename = "person_name")]
    name: String,
});
```

<br>

## Why not attribute macro?

- [You can reuse the fields with attribute macro also.](https://github.com/steadylearner/born-attribute) But, you need some dependencies.

- [If you want more, please read the official documenation about procedural macros.](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)

<br>

## How to test it

```console
$git clone git@github.com:steadylearner/born.git && cargo test pass
```

1. `$cargo install cargo-expand` and `$cargo test pass` to run passing tests.
2. `$cargo test fail` to run failing tests. You need to install [trybuild] first.

If you want to see how the macros from this package expand, use `$cargo test macros`. You need to install [rustfmt](https://github.com/rust-lang/rustfmt) and [cargo-expand](https://github.com/dtolnay/cargo-expand) to use it before.

When you use serde or others with it, cargo-expand command might show errors but that doesn't mean that the code from this package will fail.

```console
$rustup component add rustfmt && cargo install cargo-expand
```

[macrotest] is based on [trybuild]. They are not that compatible to test with a single command and take long time.

They make cargo to redownload the dependendencies and recompile everytime. For that reason, there are commands to test them separately.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
