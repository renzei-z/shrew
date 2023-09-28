use std::collections::HashMap;

use request::Request;
use response::Response;

pub mod app;
pub mod stream;
pub mod request;
pub mod response;

/// The main wrapper through which the
/// end user interacts with shrew.
pub struct App {
    routes: HashMap<String, Box<dyn FnMut(Request, Response) -> std::io::Result<()>>>
}
