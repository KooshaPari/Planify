use agileplus_mcp_intent::api::serve;
use std::net::SocketAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = std::env::var("MCP_INTENT_ADDR")
        .ok()
        .and_then(|s| SocketAddr::from_str(&s).ok())
        .unwrap_or_else(|| SocketAddr::from_str("127.0.0.1:3000").unwrap());

    let db_path = std::env::var("MCP_INTENT_DB")
        .unwrap_or_else(|_| "agileplus.db".to_string());

    println!("agileplus-mcp-intent listening on {}, db={}", addr, db_path);
    serve(addr, &db_path).await
}
