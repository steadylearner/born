# Born blog example

WIP, it will have a blog and will be used for steadylearner.com

## How to test

Use the command below to test.

### Use these when you test it in a separate project

```console
$cargo run 
$cargo watch -x run 
```

Edit Cargo.toml to use this insead.

```toml
born = { git = "https://github.com/steadylearner/born", branch = "master" }
```

### Use these when you test in born repository

```
$cargo run --example blog
```

## TODO

Make a blog post for this after the deployment of steadylearner.com
