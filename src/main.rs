use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("server") => run_server().await,
        Some("cli") => run_cli().await,
        _ => {}
    }
}

pub async fn run_cli() {
    
    
}

pub async fn run_server() {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    playground::run_playground(addr).await.unwrap();
}
