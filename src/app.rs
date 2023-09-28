use std::net::TcpListener;

use crate::App;
use crate::stream::Stream;
use crate::request::Request;
use crate::response::Response;

impl App {
    pub fn new() -> Self {
        App {
            routes: Vec::new() 
        }
    }

    pub fn register_route(&mut self, route: &str, closure: Box<dyn Fn(Request, Response) -> Response>) {
        self.routes.push(closure);
    }

    fn handle_stream(&mut self, mut stream: Stream) -> std::io::Result<()> {
        let request = match stream.get_next_request() {
            Ok(r) => r,
            Err(error) => {
                match error {
                    _ => todo!("Handle request error")
                }
            }
        };

        // Just use the first route for now, since routing is not
        // implemented.
        let route = match self.routes.pop() {
            None => panic!("No routes have been registered. Please use the command 'app.register_route' to register one before proceeding."),
            Some(r) => r
        };

        //route(request, Response::new(&mut stream));

        Ok(())
    }

    pub fn bind_localhost(&mut self, port: usize) -> std::io::Result<()> {
        let listener = TcpListener::bind(format!("[::1]:{}", port))?;

        for stream in listener.incoming() {
            self.handle_stream(Stream(&mut stream?))?;
        }

        Ok(())
    }
}