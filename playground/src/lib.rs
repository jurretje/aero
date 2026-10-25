use std::net::SocketAddr;

use axum::{Json, Router, routing::post};
use pipeline::{Compiler, StylizedToken};
use serde::{Deserialize, Serialize};
use syntax::{graph::GrammarGraph, node::Node};
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct Cursor {
    anchor: usize,
    position: usize,
}

#[derive(Debug, Deserialize)]
struct AnalysisRequest {
    source: String,
}

#[derive(Debug, Serialize)]
struct AnalysisResponse {
    success: bool,
    tokens: Vec<StylizedToken>,
    ast: Option<Node>,
    grammar_graph: Option<GrammarGraph>,
}

async fn analyze_code(Json(request): Json<AnalysisRequest>) -> Json<AnalysisResponse> {
    let comp = Compiler::new();
    let analysis = comp.analyze(&request.source);

    Json(AnalysisResponse {
        success: true,
        tokens: analysis.tokens,
        ast: analysis.parsed,
        grammar_graph: None,
    })
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
