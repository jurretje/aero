use std::net::SocketAddr;

use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use syntax::{
    lexer::{Lexer},
    source::Span,
    style::Style,
    token::{TokenKind},
};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct Cursor {
    anchor: usize,
    position: usize,
}

#[derive(Debug, Deserialize)]
struct AnalysisRequest {
    source: String,
    cursors: Vec<Cursor>,
}

#[derive(Debug, Serialize)]
struct AnalysisResponse {
    success: bool,
    tokens: Vec<StylizedToken>,
}

#[derive(Debug, Serialize)]
pub struct StylizedToken {
    kind: TokenKind,
    span: Span,
    style: Style,
}

async fn analyze_code(Json(request): Json<AnalysisRequest>) -> Json<AnalysisResponse> {
    let mut lexer = Lexer::new(&request.source);

    let tokens = lexer.tokens();
    let tokens = tokens
        .into_iter()
        .map(|token| StylizedToken {
            kind: token.kind,
            span: token.span,
            style: token.kind.style(),
        })
        .collect();

    Json(AnalysisResponse {
        success: true,
        tokens,
    })
}

pub async fn run_playground(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let frontend_dir = "./playground/frontend/";

    let app = Router::new()
        .route("/api/analysis", post(analyze_code))
        .fallback_service(ServeDir::new(frontend_dir));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    println!("Server started at http://{}", listener.local_addr()?);
    
    axum::serve(listener, app).await?;

    Ok(())
}
