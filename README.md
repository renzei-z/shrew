# SHREW - Standalone Headless Rust-Engine Webserver

SHREW is a work-in-progress web server written in Rust, without any dependencies.

## Usage

A simple web server that returns a 200 OK on `/`:

```rust
extern crate shrew;
use shrew::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.register_route("/", Box::new(|_req, res| {
        res.send_status(200)
    }));    

    app.bind_localhost(8080)?;

    Ok(())
}
```