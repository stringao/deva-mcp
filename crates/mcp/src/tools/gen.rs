use crate::error::McpError;
use crate::tools::registry::{ToolDefinition, ToolInput, ToolResult};
use rmcp::model::JsonObject;
use std::sync::Arc;

/// Create a tool definition from a name, description, schema, and executor function
pub fn make_tool(
    name: impl Into<String>,
    description: impl Into<String>,
    input_schema: JsonObject,
    executor: impl Fn(ToolInput) -> Result<ToolResult, McpError> + Send + Sync + 'static,
) -> ToolDefinition {
    ToolDefinition {
        name: name.into(),
        description: description.into(),
        input_schema,
        executor: Arc::new(executor),
    }
}
