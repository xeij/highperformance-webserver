use crate::{Response, Result, Router, ServerError};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server as HyperServer};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info, warn};

pub struct Server {
    router: Arc<Router>,
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            router: Arc::new(Router::new()),
            addr,
        }
    }

    pub fn with_router(mut self, router: Router) -> Self {
        self.router = Arc::new(router);
        self
    }

    pub async fn run(self) -> Result<()> {
        // Initialize tracing
        tracing_subscriber::fmt::init();

        info!("Starting server on {}", self.addr);

        let router = self.router.clone();

        // Create the service factory
        let make_svc = make_service_fn(move |_conn| {
            let router = router.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let router = router.clone();
                    async move { handle_request(router, req).await }
                }))
            }
        });

        // Create the server with HTTP/2 support
        let server = HyperServer::bind(&self.addr)
            .http2_only(false) // Allow both HTTP/1.1 and HTTP/2
            .http2_initial_stream_window_size(Some(1024 * 1024)) // 1MB
            .http2_initial_connection_window_size(Some(1024 * 1024 * 10)) // 10MB
            .http2_max_frame_size(Some(1024 * 64)) // 64KB
            .serve(make_svc);

        info!("Server running on http://{}", self.addr);
        info!("HTTP/2 support enabled");

        // Run the server
        if let Err(e) = server.await {
            error!("Server error: {}", e);
            return Err(ServerError::Hyper(e));
        }

        Ok(())
    }

    pub async fn run_with_graceful_shutdown<F>(self, signal: F) -> Result<()>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        // Initialize tracing
        tracing_subscriber::fmt::init();

        info!("Starting server on {} with graceful shutdown", self.addr);

        let router = self.router.clone();

        // Create the service factory
        let make_svc = make_service_fn(move |_conn| {
            let router = router.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let router = router.clone();
                    async move { handle_request(router, req).await }
                }))
            }
        });

        // Create the server with HTTP/2 support
        let server = HyperServer::bind(&self.addr)
            .http2_only(false)
            .http2_initial_stream_window_size(Some(1024 * 1024))
            .http2_initial_connection_window_size(Some(1024 * 1024 * 10))
            .http2_max_frame_size(Some(1024 * 64))
            .serve(make_svc);

        info!("Server running on http://{}", self.addr);
        info!("HTTP/2 support enabled");

        // Run the server with graceful shutdown
        let graceful = server.with_graceful_shutdown(signal);

        if let Err(e) = graceful.await {
            error!("Server error: {}", e);
            return Err(ServerError::Hyper(e));
        }

        info!("Server shutdown gracefully");
        Ok(())
    }
}

async fn handle_request(
    router: Arc<Router>,
    req: Request<Body>,
) -> std::result::Result<hyper::Response<Body>, Infallible> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    match router.handle(req).await {
        Ok(response) => match response.into_hyper_response() {
            Ok(hyper_response) => {
                info!("{} {} - 200", method, path);
                Ok(hyper_response)
            }
            Err(e) => {
                error!("Response conversion error: {}", e);
                Ok(error_response(e))
            }
        },
        Err(e) => {
            let status_code = e.status_code();
            if status_code == hyper::StatusCode::NOT_FOUND {
                warn!("{} {} - 404", method, path);
            } else {
                error!("{} {} - {} ({})", method, path, status_code.as_u16(), e);
            }
            Ok(error_response(e))
        }
    }
}

fn error_response(error: ServerError) -> hyper::Response<Body> {
    let status = error.status_code();
    let body = Body::from(format!("{{\"error\": \"{}\"}}", error));

    hyper::Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap_or_else(|_| {
            hyper::Response::builder()
                .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        })
} 