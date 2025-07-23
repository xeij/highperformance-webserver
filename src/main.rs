use high_performance_webserver::{Response, Router, Server};
use hyper::{Body, Request, StatusCode};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::signal;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create router with example routes
    let router = Router::new()
        .get("/", home_handler)
        .get("/health", health_handler)
        .get("/users", get_users_handler)
        .get("/users/1", get_user_handler)
        .post("/users", create_user_handler)
        .get("/api/stats", stats_handler)
        .get("/async-demo", async_demo_handler);

    // Server configuration
    let addr: SocketAddr = "127.0.0.1:3000".parse()?;
    let server = Server::new(addr).with_router(router);

    println!("🚀 High-Performance Web Server");
    println!("📍 Server starting on http://{}", addr);
    println!("🔗 HTTP/2 support enabled");
    println!("\n📋 Available endpoints:");
    println!("  GET  /           - Home page");
    println!("  GET  /health     - Health check");
    println!("  GET  /users      - List users");
    println!("  GET  /users/1    - Get specific user");
    println!("  POST /users      - Create user");
    println!("  GET  /api/stats  - Server statistics");
    println!("  GET  /async-demo - Async operation demo");
    println!("\n⏳ Press Ctrl+C to shutdown gracefully...\n");

    // Run server with graceful shutdown
    server.run_with_graceful_shutdown(shutdown_signal()).await?;

    Ok(())
}

async fn home_handler(_req: Request<Body>) -> high_performance_webserver::Result<Response> {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>High-Performance Web Server</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }
            .container { max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
            h1 { color: #333; text-align: center; }
            .endpoint { background: #f8f9fa; padding: 15px; margin: 10px 0; border-radius: 5px; border-left: 4px solid #007bff; }
            .method { font-weight: bold; color: #007bff; }
            .feature { background: #e8f5e8; padding: 10px; margin: 5px 0; border-radius: 5px; }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>🚀 High-Performance Web Server</h1>
            <p>Built with Rust, featuring async I/O and HTTP/2 support!</p>
            
            <h2>🔥 Key Features</h2>
            <div class="feature">⚡ Async I/O with Tokio runtime</div>
            <div class="feature">🌐 HTTP/2 and HTTP/1.1 support</div>
            <div class="feature">🛣️ Flexible routing system</div>
            <div class="feature">📊 JSON API responses</div>
            <div class="feature">🎯 High-performance architecture</div>
            
            <h2>📋 API Endpoints</h2>
            <div class="endpoint">
                <span class="method">GET</span> /health - Health check
            </div>
            <div class="endpoint">
                <span class="method">GET</span> /users - List all users
            </div>
            <div class="endpoint">
                <span class="method">GET</span> /users/1 - Get specific user
            </div>
            <div class="endpoint">
                <span class="method">POST</span> /users - Create new user
            </div>
            <div class="endpoint">
                <span class="method">GET</span> /api/stats - Server statistics
            </div>
            <div class="endpoint">
                <span class="method">GET</span> /async-demo - Async operation demo
            </div>
        </div>
    </body>
    </html>
    "#;

    Ok(Response::new().html(html))
}

async fn health_handler(_req: Request<Body>) -> high_performance_webserver::Result<Response> {
    let response = ApiResponse {
        success: true,
        data: "Server is healthy and running!",
        message: "All systems operational".to_string(),
    };

    Response::new().json(&response)
}

async fn get_users_handler(_req: Request<Body>) -> high_performance_webserver::Result<Response> {
    let users = vec![
        User {
            id: 1,
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
        },
        User {
            id: 3,
            name: "Carol Davis".to_string(),
            email: "carol@example.com".to_string(),
        },
    ];

    let response = ApiResponse {
        success: true,
        data: users,
        message: "Users retrieved successfully".to_string(),
    };

    Response::new().json(&response)
}

async fn get_user_handler(_req: Request<Body>) -> high_performance_webserver::Result<Response> {
    let user = User {
        id: 1,
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
    };

    let response = ApiResponse {
        success: true,
        data: user,
        message: "User retrieved successfully".to_string(),
    };

    Response::new().json(&response)
}

async fn create_user_handler(_req: Request<Body>) -> high_performance_webserver::Result<Response> {
    // In a real application, you would parse the request body
    let user = User {
        id: 4,
        name: "New User".to_string(),
        email: "newuser@example.com".to_string(),
    };

    let response = ApiResponse {
        success: true,
        data: user,
        message: "User created successfully".to_string(),
    };

    Response::new()
        .status(StatusCode::CREATED)
        .json(&response)
}

async fn stats_handler(_req: Request<Body>) -> high_performance_webserver::Result<Response> {
    #[derive(Serialize)]
    struct ServerStats {
        uptime: String,
        memory_usage: String,
        active_connections: u32,
        total_requests: u64,
        http2_enabled: bool,
    }

    let stats = ServerStats {
        uptime: "Running".to_string(),
        memory_usage: "Optimized".to_string(),
        active_connections: 1,
        total_requests: 42,
        http2_enabled: true,
    };

    let response = ApiResponse {
        success: true,
        data: stats,
        message: "Server statistics retrieved".to_string(),
    };

    Response::new().json(&response)
}

async fn async_demo_handler(_req: Request<Body>) -> high_performance_webserver::Result<Response> {
    // Simulate an async operation
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    #[derive(Serialize)]
    struct AsyncResult {
        operation: String,
        duration_ms: u64,
        result: String,
    }

    let result = AsyncResult {
        operation: "Async computation".to_string(),
        duration_ms: 100,
        result: "Completed successfully with async I/O!".to_string(),
    };

    let response = ApiResponse {
        success: true,
        data: result,
        message: "Async operation completed".to_string(),
    };

    Response::new().json(&response)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("\n🛑 Shutdown signal received, starting graceful shutdown...");
} 