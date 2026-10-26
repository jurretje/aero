use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1).as_deref() {
        Some("playground") => {
            let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 3300));
            playground::run_playground(address).await?
        }
        _ => cli::run_cli()?,
    }

    Ok(())
}
