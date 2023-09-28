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
