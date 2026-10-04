use std::net::SocketAddr;

use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use syntax::{
    lexer::{self, Lexer},
    source::Span,
    style::Style,
    token::{Token, TokenKind},
};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct Cursor {
    anchor: usize,
    position: usize,
}

#[derive(Debug, Deserialize)]
struct CompileRequest {
    source: String,
    cursors: Vec<Cursor>,
}

#[derive(Debug, Serialize)]
struct CompileResponse {
    success: bool,
    tokens: Vec<StylizedToken>,
}

#[derive(Debug, Serialize)]
pub struct StylizedToken {
    kind: TokenKind,
    span: Span,
    style: Style,
}

async fn compile(Json(request): Json<CompileRequest>) -> Json<CompileResponse> {
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

    Json(CompileResponse {
        success: true,
        tokens,
    })
}

pub async fn run_playground(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {

    let app = Router::new()
        .route("/api/compile", post(compile))
        .fallback_service(ServeDir::new("./playground/frontend"));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    println!("Server started at http://{}", listener.local_addr()?);
    
    axum::serve(listener, app).await?;

    Ok(())
}
