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
struct AnalysisRequest {
    source: String,
    cursors: Vec<Cursor>,
}

#[derive(Debug, Serialize)]
struct AnalysisResult {
    success: bool,
    tokens: Vec<StylizedToken>,
    ast: Option<Node>,
    grammar_graph: Option<GrammarGraph>,
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            success: false,
            tokens: vec![],
            ast: None,
            grammar_graph: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StylizedToken {
    kind: TokenKind,
    span: Span,
    style: Style,
}

async fn analyze_code(Json(request): Json<AnalysisRequest>) -> Json<AnalysisResult> {
    let mut lexer = Lexer::new(&request.source);

    let tokens = lexer.tokens();
    let styled_tokens = tokens
        .iter()
        .map(|token| StylizedToken {
            kind: token.kind,
            span: token.span.clone(),
            style: token.kind.style(),
        })
        .collect();

    let grammar = build_grammar();
    let graph = GrammarGraphBuilder::new(&grammar).build();
    // let Some(actions) = parse(
    //     &graph,
    //     &tokens,
    //     ParseCosts {
    //         insert: 2,
    //         delete: 4,
    //         replace: 5,
    //     },
    // ) else {
    //     return Json(AnalysisResult::default());
    // };

    // let Some(ast) = build_ast(&actions, &tokens) else {
    //     return Json(AnalysisResult::default());
    // };

    Json(AnalysisResult {
        success: true,
        tokens: styled_tokens,
        ast: None,
        grammar_graph: Some(graph),
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
