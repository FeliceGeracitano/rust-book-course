use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr: SocketAddr = std::env::var("DEMO_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:9090".into())
        .parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("demo-upstream listening on http://{addr}");
    axum::serve(listener, demo_upstream::router()).await?;
    Ok(())
}
