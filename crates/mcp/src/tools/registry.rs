use crate::error::McpError;
use rmcp::model::{JsonObject, Tool};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Input for tool execution
#[derive(Debug, Clone)]
pub struct ToolInput {
    pub name: String,
    pub arguments: Value,
}

impl ToolInput {
    pub fn new(name: String, arguments: Value) -> Self {
        Self { name, arguments }
    }
}

/// Result of tool execution
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub content: Vec<Value>,
}

impl ToolResult {
    pub fn new(content: Vec<Value>) -> Self {
        Self { content }
    }

    pub fn from_json(value: Value) -> Self {
        Self {
            content: vec![value],
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            content: vec![serde_json::json!({ "error": message })],
        }
    }
}

/// Tool executor function type
pub type ToolExecutor = Arc<dyn Fn(ToolInput) -> Result<ToolResult, McpError> + Send + Sync>;

/// Tool definition
#[derive(Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: JsonObject,
    pub executor: ToolExecutor,
}

/// Tool registry for managing MCP tools
#[derive(Clone)]
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, ToolDefinition>>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new tool
    pub async fn register<F>(
        &self,
        name: String,
        description: String,
        input_schema: JsonObject,
        executor: F,
    ) where
        F: Fn(ToolInput) -> Result<ToolResult, McpError> + Send + Sync + 'static,
    {
        let def = ToolDefinition {
            name: name.clone(),
            description,
            input_schema,
            executor: Arc::new(executor),
        };
        self.tools.write().await.insert(name, def);
    }

    /// List all registered tools
    pub async fn list(&self) -> Vec<Tool> {
        let tools = self.tools.read().await;
        tools
            .values()
            .map(|def| Tool::new(
                def.name.clone(),
                def.description.clone(),
                Arc::new(def.input_schema.clone()),
            ))
            .collect()
    }

    /// Execute a tool by name
    pub async fn execute(&self, name: &str, input: ToolInput) -> Result<ToolResult, McpError> {
        let tools = self.tools.read().await;
        let def = tools
            .get(name)
            .ok_or_else(|| McpError::ToolNotFound(name.to_string()))?;
        (def.executor)(input)
    }

    /// Get the count of registered tools
    pub async fn count(&self) -> usize {
        self.tools.read().await.len()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
