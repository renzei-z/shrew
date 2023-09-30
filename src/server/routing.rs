use super::*;

pub type RouteResult<'r> = std::io::Result<Response<'r>>;

fn match_uri(uri: &str, route: &str) -> bool {
    // For now, just check if they match:
    uri == route
}

pub fn default_fallback_route(_req: Request, res: Response) -> RouteResult<'_> {
    res.set_status(404).send("404 Not Found")
}

impl Server {
    pub fn get(&mut self, route: &str, response: RoutingFunction) -> ServerResult {
        if self.routes.iter().any(|r| r.0 == route.to_string()) {
            return Err(ServerError::DuplicateRoute(route.to_string()));
        }
        self.routes.push((route.to_string(), response));
        Ok(())
    }

    pub(crate) fn route<'r>(&'r mut self, req: Request, res: Response<'r>) -> RouteResult<'r> {
        // TODO: Caching?
        match self.routes.iter().find(|r| match_uri(&req.request_uri, &r.0)) {
            Some((_route, routing_function)) => (routing_function)(req, res),
            None => (self.fallback_route)(req, res)
        }
    }
}