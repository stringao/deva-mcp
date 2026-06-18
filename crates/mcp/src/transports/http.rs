//! HTTP transport for MCP server.
//!
//! This module provides HTTP-based MCP server transport.

use crate::server::DevaMcpServer;
use crate::tools::ToolInput;
use anyhow::Result;
use std::sync::Arc;
use tracing::info;

/// Run MCP server with HTTP transport
pub async fn run_http(port: u16) -> Result<()> {
    let server = DevaMcpServer::new();
    server.register_tools().await;
    let addr = format!("0.0.0.0:{}", port);

    info!("MCP HTTP server listening on {}", addr);

    let app = axum::Router::new()
        .route("/mcp", axum::routing::post(handle_mcp_request))
        .route("/health", axum::routing::get(|| async { "ok" }))
        .with_state(Arc::new(server));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    params: Option<serde_json::Value>,
}

async fn handle_mcp_request(
    axum::extract::State(server): axum::extract::State<Arc<DevaMcpServer>>,
    axum::Json(request): axum::Json<JsonRpcRequest>,
) -> axum::Json<serde_json::Value> {
    let request_id = request.id.clone();

    match request.method.as_str() {
        "tools/list" => {
            let tools = server.registry().list().await;
            let tools_json: Vec<_> = tools.into_iter().map(|t| serde_json::json!({
                "name": t.name,
                "description": t.description,
                "inputSchema": t.input_schema
            })).collect();
            axum::Json(serde_json::json!({
                "jsonrpc": "2.0",
                "id": request_id,
                "result": { "tools": tools_json }
            }))
        }
        "tools/call" => {
            let params = request.params.unwrap_or(serde_json::Value::Null);
            let name = params.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let arguments = params.get("arguments")
                .unwrap_or(&serde_json::Value::Null)
                .clone();

            let input = ToolInput::new(name.clone(), arguments);

            match server.registry().execute(&name, input).await {
                Ok(result) => {
                    let content: Vec<_> = result.content.into_iter().map(|v| {
                        serde_json::json!({ "type": "text", "text": v.to_string() })
                    }).collect();
                    axum::Json(serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "result": { "content": content }
                    }))
                }
                Err(e) => axum::Json(serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "error": { "code": -32603, "message": e.to_string() }
                })),
            }
        }
        _ => axum::Json(serde_json::json!({
            "jsonrpc": "2.0",
            "id": request_id,
            "error": { "code": -32601, "message": "Method not found" }
        })),
    }
}
