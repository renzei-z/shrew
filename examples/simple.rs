extern crate shrew;
use shrew::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.register_route("/", Box::new(|_req, res| {
        res.set_header("Location", "https://google.se").send_status(307)
    }));

    app.bind_localhost(8080)?;

    Ok(())
}
