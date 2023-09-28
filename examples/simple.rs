extern crate shrew;
use shrew::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.register_route("/", Box::new(|_req, res| {
        res.send(b"Hello, world!")
    }));

    app.register_route("/test", Box::new(|_req, res| {
        res.send(b"Test Route!")
    }));

    app.bind_localhost(8080)?;

    Ok(())
}
