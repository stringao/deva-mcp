//! Stdio transport for MCP server.
//!
//! Note: rmcp's stdio support is primarily designed for client connections.
//! For server-side stdio (e.g., for Claude Code integration), we use
//! the HTTP/WebSocket transport internally.

use anyhow::Result;

/// Run MCP server with HTTP transport (stdio not directly supported by rmcp server).
/// For Claude Code integration, use the HTTP endpoint or configure a reverse proxy.
pub async fn run_stdio() -> Result<()> {
    // For now, redirect to HTTP on a default port
    // A proper stdio server would require custom transport implementation
    super::http::run_http(8081).await
}
