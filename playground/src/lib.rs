use std::net::SocketAddr;

use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use syntax::{
    grammar::{Grammar, build_grammar},
    graph::{GrammarGraph, GrammarGraphBuilder},
    lexer::Lexer,
    node::Node,
    parser::{ParseCosts, build_ast, parse},
    source::Span,
    style::Style,
    token::TokenKind,
};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct Cursor {
    anchor: usize,
    position: usize,
}

#[derive(Debug, Deserialize)]
struct AnalysisRequest {}

#[derive(Debug, Serialize)]
struct AnalysisResponse {}

async fn analyze_code(Json(request): Json<AnalysisRequest>) -> Json<AnalysisResponse> {
    todo!()
}

#[derive(Deserialize)]
pub struct CompileRequest {}

#[derive(Serialize)]
pub struct CompileResponse {}

async fn compile_code(Json(request): Json<CompileRequest>) -> Json<CompileResponse> {
    Json(CompileResponse {})
}

pub async fn run_playground(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let frontend_dir = "./playground/frontend/";

    let app = Router::new()
        .route("/api/analysis", post(analyze_code))
        .route("/api/compile", post(compile_code))
        .fallback_service(ServeDir::new(frontend_dir));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Server started at http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
