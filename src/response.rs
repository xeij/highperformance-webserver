use hyper::{Body, StatusCode};
use serde::Serialize;
use std::collections::HashMap;

pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Body,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HashMap::new(),
            body: Body::empty(),
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn body<B>(mut self, body: B) -> Self
    where
        B: Into<Body>,
    {
        self.body = body.into();
        self
    }

    pub fn text<S>(self, text: S) -> Self
    where
        S: Into<String>,
    {
        self.header("Content-Type", "text/plain")
            .body(Body::from(text.into()))
    }

    pub fn html<S>(self, html: S) -> Self
    where
        S: Into<String>,
    {
        self.header("Content-Type", "text/html")
            .body(Body::from(html.into()))
    }

    pub fn json<T>(self, value: &T) -> crate::Result<Self>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(value)?;
        Ok(self
            .header("Content-Type", "application/json")
            .body(Body::from(json)))
    }

    pub(crate) fn into_hyper_response(self) -> crate::Result<hyper::Response<Body>> {
        let mut response = hyper::Response::builder().status(self.status);

        for (key, value) in self.headers {
            response = response.header(key, value);
        }

        Ok(response.body(self.body)?)
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
} 