use crate::error::McpError;
use crate::tools::registry::{ToolInput, ToolResult};
use deva_azure_devops::AzureDevOpsClient;

/// Executor for Azure DevOps tools
#[derive(Clone)]
pub struct AzureToolExecutor {
    client: AzureDevOpsClient,
}

impl AzureToolExecutor {
    /// Create a new executor with an Azure DevOps client
    pub fn new(client: AzureDevOpsClient) -> Self {
        Self { client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self, McpError> {
        let client = AzureDevOpsClient::from_env()
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(Self::new(client))
    }

    // ============ Work Items ============

    /// Execute azure_workitem_list
    pub async fn list_work_items_async(&self, work_item_type: Option<&str>, state: Option<&str>) -> Result<ToolResult, McpError> {
        let items = self.client.list_work_items(work_item_type, state)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!({
            "work_items": items,
            "count": items.len()
        })))
    }

    /// Execute azure_workitem_get
    pub async fn get_work_item_async(&self, id: u64) -> Result<ToolResult, McpError> {
        let item = self.client.get_work_item(id)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!(item)))
    }

    // ============ Sprints ============

    /// Execute azure_sprint_list
    pub async fn list_sprints_async(&self) -> Result<ToolResult, McpError> {
        let sprints = self.client.list_sprints()
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!({
            "sprints": sprints,
            "count": sprints.len()
        })))
    }

    // ============ Boards ============

    /// Execute azure_board_get
    pub async fn get_board_async(&self, board_name: &str) -> Result<ToolResult, McpError> {
        let board = self.client.get_board(board_name)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!(board)))
    }
}