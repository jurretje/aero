use std::net::{Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));
    playground::run_playground(addr).await.unwrap();
}
