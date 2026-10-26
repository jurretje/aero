use std::{net::SocketAddr, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use pipeline::{Compiler, CompilerOptions, StylizedToken};
use serde::{Deserialize, Serialize};
use syntax::{graph::GrammarGraph, node::Node, theme::Theme};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct Cursor {
    anchor: usize,
    position: usize,
}

#[derive(Clone)]
struct AppState {
    compiler: Arc<RwLock<Compiler>>,
    initial_source: Arc<str>,
}

pub async fn run_playground(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let frontend_dir = "./playground/frontend/";

    let initial_source: Arc<str> = Arc::from(std::fs::read_to_string("examples/main.ae")?);

    let state = AppState {
        compiler: Arc::new(RwLock::new(Compiler::new())),
        initial_source,
    };

    let app = Router::new()
        .route("/api/init", get(initialize))
        .route("/api/analysis", post(analyze_code))
        .route("/api/compile", post(compile_code))
        .route("/api/options", get(get_options).put(update_options))
        .fallback_service(ServeDir::new(frontend_dir))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Server started at http://{}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct InitResponse {
    source: String,
    options: CompilerOptions,
    theme: Theme,
}

async fn initialize(State(state): State<AppState>) -> Result<Json<InitResponse>, StatusCode> {
    let compiler = state.compiler.read().await;
    let options = compiler.options.clone();
    let theme = options.theme.build();

    Ok(Json(InitResponse {
        source: state.initial_source.to_string(),
        options,
        theme,
    }))
}

#[derive(Debug, Deserialize)]
struct OptionsRequest {
    options: CompilerOptions,
}

#[derive(Debug, Serialize)]
struct OptionsResponse {
    options: CompilerOptions,
    theme: Theme,
}

async fn get_options(State(state): State<AppState>) -> Json<OptionsResponse> {
    let compiler = state.compiler.read().await;

    let options = compiler.options.clone();
    let theme = options.theme.build();

    Json(OptionsResponse { options, theme })
}

async fn update_options(
    State(state): State<AppState>,
    Json(request): Json<OptionsRequest>,
) -> Result<Json<OptionsResponse>, StatusCode> {
    let mut compiler = state.compiler.write().await;
    compiler.options = request.options;

    let options = compiler.options.clone();
    let theme = options.theme.build();

    Ok(Json(OptionsResponse { options, theme }))
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
    ast: Option<Node>,
    grammar_graph: Option<GrammarGraph>,
}

async fn analyze_code(
    State(state): State<AppState>,
    Json(request): Json<AnalysisRequest>,
) -> Result<Json<AnalysisResponse>, StatusCode> {
    let comp = state.compiler.read().await;
    let analysis = comp.analyze(&request.source);

    Ok(Json(AnalysisResponse {
        success: true,
        tokens: analysis.tokens,
        ast: analysis.parsed,
        grammar_graph: None,
    }))
}

#[derive(Deserialize)]
pub struct CompileRequest {}

#[derive(Serialize)]
pub struct CompileResponse {}

async fn compile_code(Json(_request): Json<CompileRequest>) -> Json<CompileResponse> {
    Json(CompileResponse {})
}
