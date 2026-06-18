use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let transport = std::env::var("MCP_TRANSPORT").unwrap_or_else(|_| "stdio".into());

    match transport.as_str() {
        "http" => {
            let port: u16 = std::env::var("MCP_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()?;
            deva_mcp::transports::http::run_http(port).await?;
        }
        _ => {
            deva_mcp::transports::stdio::run_stdio().await?;
        }
    }

    Ok(())
}
