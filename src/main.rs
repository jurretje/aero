use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1).as_deref() {
        Some("server") => run_server().await,
        _ => cli::run_cli()?,
    }

    Ok(())
}

pub async fn run_server() {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    playground::run_playground(addr).await.unwrap();
}
