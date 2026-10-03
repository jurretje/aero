use std::net::SocketAddr;

use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use syntax::lexer::{self, Lexer};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct Cursor {
    anchor: usize,
    position: usize
}

#[derive(Debug, Deserialize)]
struct CompileRequest {
    source: String,
    cursors: Vec<Cursor>,
}

#[derive(Debug, Serialize)]
struct CompileResponse {
    success: bool,
}

async fn compile(Json(request): Json<CompileRequest>) -> Json<CompileResponse> {
    println!("Source:\n{}", request.source);
    println!("Cursors: {:?}", request.cursors);

    let mut lexer = Lexer::new(&request.source);

    let tokens = lexer.tokens();

    println!("{:?}", tokens);

    Json(CompileResponse { success: true })
}

pub async fn run_playground(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/api/compile", post(compile))
        .fallback_service(ServeDir::new("./playground/frontend"));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
