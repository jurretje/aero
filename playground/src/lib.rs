use std::net::SocketAddr;

use axum::Router;
use tower_http::services::ServeDir;

pub async fn run_playground(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().fallback_service(ServeDir::new("./playground/frontend"));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
