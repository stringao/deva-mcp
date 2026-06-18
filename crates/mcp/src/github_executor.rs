use crate::error::McpError;
use crate::tools::registry::{ToolInput, ToolResult};
use deva_github::GitHubClient;
use serde_json::Value;

/// Executor for GitHub tools
#[derive(Clone)]
pub struct GitHubToolExecutor {
    client: GitHubClient,
}

impl GitHubToolExecutor {
    /// Create a new executor with a GitHub client
    pub fn new(client: GitHubClient) -> Self {
        Self { client }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self, McpError> {
        let client = GitHubClient::from_env()
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(Self::new(client))
    }

    /// Execute github_pr_list (placeholder)
    pub fn list_prs(&self, input: ToolInput) -> Result<ToolResult, McpError> {
        let _state = input.arguments.get("state")
            .and_then(|v| v.as_str());
        Ok(ToolResult::from_json(serde_json::json!({
            "status": "placeholder",
            "prs": []
        })))
    }

    /// Execute github_pr_list with real API call
    pub async fn list_prs_async(&self, state: Option<&str>) -> Result<ToolResult, McpError> {
        let prs = self.client.list_prs(state)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!({
            "pull_requests": prs,
            "count": prs.len()
        })))
    }

    /// Execute github_pr_get (placeholder)
    pub fn get_pr(&self, input: ToolInput) -> Result<ToolResult, McpError> {
        let pr_number = input.arguments.get("pr_number")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| McpError::InvalidInput("pr_number is required".into()))?;
        Ok(ToolResult::from_json(serde_json::json!({
            "status": "placeholder",
            "pr_number": pr_number
        })))
    }

    /// Execute github_pr_create (placeholder)
    pub fn create_pr(&self, input: ToolInput) -> Result<ToolResult, McpError> {
        let title = input.arguments.get("title")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::InvalidInput("title is required".into()))?;
        let _head = input.arguments.get("head")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::InvalidInput("head is required".into()))?;
        let _base = input.arguments.get("base")
            .and_then(|v| v.as_str())
            .unwrap_or("main");
        let _body = input.arguments.get("body")
            .and_then(|v| v.as_str());
        Ok(ToolResult::from_json(serde_json::json!({
            "status": "placeholder",
            "title": title
        })))
    }

    /// Execute github_issue_list (placeholder)
    pub fn list_issues(&self, _input: ToolInput) -> Result<ToolResult, McpError> {
        Ok(ToolResult::from_json(serde_json::json!({
            "status": "placeholder",
            "issues": []
        })))
    }

    /// Execute github_issue_list with real API call
    pub async fn list_issues_async(&self, state: Option<&str>) -> Result<ToolResult, McpError> {
        let issues = self.client.list_issues(state, None)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!({
            "issues": issues,
            "count": issues.len()
        })))
    }

    /// Execute github_branch_list (placeholder)
    pub fn list_branches(&self, _input: ToolInput) -> Result<ToolResult, McpError> {
        Ok(ToolResult::from_json(serde_json::json!({
            "status": "placeholder",
            "branches": []
        })))
    }

    /// Execute github_branch_list with real API call
    pub async fn list_branches_async(&self) -> Result<ToolResult, McpError> {
        let branches = self.client.list_branches()
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!({
            "branches": branches,
            "count": branches.len()
        })))
    }

    /// Execute github_repo_get (placeholder)
    pub fn get_repo(&self, _input: ToolInput) -> Result<ToolResult, McpError> {
        Ok(ToolResult::from_json(serde_json::json!({
            "status": "placeholder"
        })))
    }

    /// Execute github_repo_get with real API call
    pub async fn get_repo_async(&self) -> Result<ToolResult, McpError> {
        let repo = self.client.get_repo()
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!(repo)))
    }

    /// Execute github_pr_get with real API call
    pub async fn get_pr_async(&self, pr_number: u64) -> Result<ToolResult, McpError> {
        let pr = self.client.get_pr(pr_number)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!(pr)))
    }

    /// Execute github_pr_create with real API call
    pub async fn create_pr_async(&self, title: &str, head: &str, base: &str, body: Option<&str>) -> Result<ToolResult, McpError> {
        let pr = self.client.create_pr(title, head, base, body)
            .await
            .map_err(|e| McpError::Internal(e.to_string()))?;
        Ok(ToolResult::from_json(serde_json::json!(pr)))
    }
}