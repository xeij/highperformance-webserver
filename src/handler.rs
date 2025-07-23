use crate::{Response, Result};
use hyper::{Body, Request};
use std::future::Future;
use std::pin::Pin;

pub type HandlerFn = Box<
    dyn Fn(Request<Body>) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        + Send
        + Sync,
>;

pub trait Handler: Send + Sync + 'static {
    fn call(&self, req: Request<Body>) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>>;
}

impl<F, Fut> Handler for F
where
    F: Fn(Request<Body>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response>> + Send + 'static,
{
    fn call(&self, req: Request<Body>) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>> {
        Box::pin(self(req))
    }
}

// Convenience macros for creating handlers
#[macro_export]
macro_rules! handler {
    ($func:expr) => {
        Box::new($func) as $crate::HandlerFn
    };
}

// Request context with path parameters
pub struct RequestContext {
    pub params: std::collections::HashMap<String, String>,
    pub query: std::collections::HashMap<String, String>,
}

impl RequestContext {
    pub fn new() -> Self {
        Self {
            params: std::collections::HashMap::new(),
            query: std::collections::HashMap::new(),
        }
    }

    pub fn param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    pub fn query_param(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }
} 