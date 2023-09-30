extern crate shrew;
use shrew::prelude::*;

fn index(_req: Request, res: Response) -> RouteResult<'_> {
    res.set_status(200).send("200 OK")
}

fn item(req: Request, res: Response) -> RouteResult<'_> {
    match req.params.get("id") {
        Some(id) if id.parse::<usize>().unwrap() < 5 => res.send(&format!("Found item {id}")),
        _ => res.set_status(404).send("404 Not Found")
    }
}

fn on_listen() {
    println!("[INFO] Listening on port 80...");
}

fn main() -> ServerResult {
    let mut server = Server::new();

    server.get("/", index)?;
    server.get("/%id", item)?;

    server.localhost(80, on_listen)
}
