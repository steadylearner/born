# Born blog example

It will have a blog example based on this? and you will be able to find a production example at steadylearner.com

## How to test

Make a .env file with this https://dev.to/settings/extensions and you can make a credential.

```env
DEV_TO=<YOURS>
```

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

Make a custom repo to deploy steadylearner.com with a frontend framework and update this example with born? Write a blog post from server to deployment without the frontend?
