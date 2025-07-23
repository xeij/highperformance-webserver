use crate::{Handler, HandlerFn, Response, Result, ServerError};
use hyper::{Body, Method as HttpMethod, Request};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl From<&HttpMethod> for Method {
    fn from(method: &HttpMethod) -> Self {
        match *method {
            HttpMethod::GET => Method::GET,
            HttpMethod::POST => Method::POST,
            HttpMethod::PUT => Method::PUT,
            HttpMethod::DELETE => Method::DELETE,
            HttpMethod::PATCH => Method::PATCH,
            HttpMethod::HEAD => Method::HEAD,
            HttpMethod::OPTIONS => Method::OPTIONS,
            _ => Method::GET, // Default fallback
        }
    }
}

pub struct Route {
    method: Method,
    path: String,
    handler: HandlerFn,
}

impl Route {
    pub fn new<H>(method: Method, path: impl Into<String>, handler: H) -> Self
    where
        H: Handler,
    {
        let handler_fn = Box::new(move |req: Request<Body>| {
            Box::pin(handler.call(req)) as Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        });

        Self {
            method,
            path: path.into(),
            handler: handler_fn,
        }
    }
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    pub fn get<H>(mut self, path: impl Into<String>, handler: H) -> Self
    where
        H: Handler,
    {
        self.routes.push(Route::new(Method::GET, path, handler));
        self
    }

    pub fn post<H>(mut self, path: impl Into<String>, handler: H) -> Self
    where
        H: Handler,
    {
        self.routes.push(Route::new(Method::POST, path, handler));
        self
    }

    pub fn put<H>(mut self, path: impl Into<String>, handler: H) -> Self
    where
        H: Handler,
    {
        self.routes.push(Route::new(Method::PUT, path, handler));
        self
    }

    pub fn delete<H>(mut self, path: impl Into<String>, handler: H) -> Self
    where
        H: Handler,
    {
        self.routes.push(Route::new(Method::DELETE, path, handler));
        self
    }

    pub fn patch<H>(mut self, path: impl Into<String>, handler: H) -> Self
    where
        H: Handler,
    {
        self.routes.push(Route::new(Method::PATCH, path, handler));
        self
    }

    pub fn route<H>(mut self, method: Method, path: impl Into<String>, handler: H) -> Self
    where
        H: Handler,
    {
        self.routes.push(Route::new(method, path, handler));
        self
    }

    pub async fn handle(&self, req: Request<Body>) -> Result<Response> {
        let method = Method::from(req.method());
        let path = req.uri().path();

        // Simple path matching for now - can be enhanced with parameters later
        for route in &self.routes {
            if route.method == method && self.path_matches(&route.path, path) {
                return (route.handler)(req).await;
            }
        }

        Err(ServerError::RouteNotFound {
            method: format!("{:?}", method),
            path: path.to_string(),
        })
    }

    fn path_matches(&self, route_path: &str, request_path: &str) -> bool {
        // Simple exact match for now
        // TODO: Add support for path parameters like /users/:id
        route_path == request_path
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
} 