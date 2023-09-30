# SHREW - Standalone Headless Rust-Engine Webserver

SHREW is a work-in-progress web server written in Rust, without any dependencies.

## Usage

A simple web server that returns a 200 OK on `/`:

```rust
extern crate shrew;
use shrew::prelude::*;

fn index(_req: Request, res: Response) -> RouteResult<'_> {
    res.set_status(200).send("200 OK")
}

fn on_listen() {
    println!("[INFO] Listening on port 80...");
}

fn main() -> ServerResult {
    let mut server = Server::new();

    server.get("/", index)?;

    server.localhost(80, on_listen)
}
```