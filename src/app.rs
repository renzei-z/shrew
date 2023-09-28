use std::net::TcpListener;

use std::collections::HashMap;

use crate::App;
use crate::stream::Stream;
use crate::request::Request;
use crate::response::Response;

impl App {
    pub fn new() -> Self {
        App {
            routes: HashMap::new() 
        }
    }

    pub fn register_route(&mut self, route: &str, closure: Box<dyn FnMut(Request, Response) -> std::io::Result<()>>) {
        // TODO: matching
        self.routes.insert(route.to_string(), closure);
    }

    fn handle_stream(&mut self, mut stream: Stream) -> std::io::Result<()> {
        let request = match stream.get_next_request() {
            Ok(r) => r,
            Err(error) => {
                match error {
                    _ => todo!("Handle request error")
                }
            }
        }.unwrap();

        match self.routes.get_mut(request.request_uri.as_str()) {
            None => Response::new(stream).send_status(404),
            Some(route) => route(request, Response::new(stream))
        }
    }

    pub fn bind_localhost(&mut self, port: usize) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("[::1]:{}", port))?;

        for stream in listener.incoming() {
            self.handle_stream(Stream(&mut stream?))?;
        }

        Ok(())
    }
}
