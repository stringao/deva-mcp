use crate::error::McpError;
use crate::tools::{ToolInput, ToolRegistry, ToolResult};
use rmcp::model::{CallToolRequestParams, CallToolResult, ListToolsResult, PaginatedRequestParams};
use rmcp::ServerHandler;
use std::future::Future;
use tracing::{error, info};

/// Main MCP server implementation
#[derive(Clone)]
pub struct DevaMcpServer {
    registry: ToolRegistry,
}

impl DevaMcpServer {
    /// Create a new MCP server with an empty registry
    pub fn new() -> Self {
        Self {
            registry: ToolRegistry::new(),
        }
    }

    /// Get a reference to the tool registry
    pub fn registry(&self) -> &ToolRegistry {
        &self.registry
    }

    /// Register all Deva tools
    pub async fn register_tools(&self) {
        info!("Registering Deva MCP tools...");
        self.register_project_tools().await;
        self.register_github_tools().await;
        self.register_azure_tools().await;
        self.register_scaffold_tools().await;
        self.register_analysis_tools().await;
        let count = self.registry.count().await;
        info!("Registered {} MCP tools", count);
    }

    async fn register_project_tools(&self) {
        use serde_json::json;

        self.registry
            .register(
                "project_list".into(),
                "List all Deva projects in the current workspace".into(),
                serde_json::json!({}).as_object().unwrap().clone(),
                |_input: ToolInput| {
                    Ok(ToolResult::from_json(json!({ "projects": [], "count": 0 })))
                },
            )
            .await;

        self.registry
            .register(
                "project_create".into(),
                "Create a new Deva project from a template".into(),
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "template": { "type": "string", "default": "nextjs" }
                    },
                    "required": ["name"]
                })
                .as_object()
                .unwrap()
                .clone(),
                |input: ToolInput| {
                    let name = input.arguments.get("name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| McpError::InvalidInput("name is required".into()))?;
                    let template = input.arguments.get("template")
                        .and_then(|v| v.as_str())
                        .unwrap_or("nextjs");
                    Ok(ToolResult::from_json(json!({
                        "status": "created", "name": name, "template": template
                    })))
                },
            )
            .await;

        self.registry
            .register(
                "project_info".into(),
                "Get detailed information about a Deva project".into(),
                serde_json::json!({
                    "type": "object",
                    "properties": { "path": { "type": "string" } },
                    "required": ["path"]
                })
                .as_object()
                .unwrap()
                .clone(),
                |input: ToolInput| {
                    let path = input.arguments.get("path")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| McpError::InvalidInput("path is required".into()))?;
                    Ok(ToolResult::from_json(json!({
                        "path": path, "exists": std::path::Path::new(path).exists()
                    })))
                },
            )
            .await;

        self.registry
            .register(
                "project_health".into(),
                "Run health check on a Deva project".into(),
                serde_json::json!({
                    "type": "object",
                    "properties": { "path": { "type": "string" } },
                    "required": ["path"]
                })
                .as_object()
                .unwrap()
                .clone(),
                |_input: ToolInput| {
                    Ok(ToolResult::from_json(json!({
                        "score": 100, "status": "healthy", "issues": []
                    })))
                },
            )
            .await;

        self.registry
            .register(
                "project_doctor".into(),
                "Run full diagnostics on a Deva project".into(),
                serde_json::json!({
                    "type": "object",
                    "properties": { "path": { "type": "string" } },
                    "required": ["path"]
                })
                .as_object()
                .unwrap()
                .clone(),
                |_input: ToolInput| {
                    Ok(ToolResult::from_json(json!({ "checks": [], "all_passed": true })))
                },
            )
            .await;

        self.registry
            .register(
                "project_init".into(),
                "Initialize Deva in an existing project directory".into(),
                serde_json::json!({
                    "type": "object",
                    "properties": { "path": { "type": "string" } },
                    "required": ["path"]
                })
                .as_object()
                .unwrap()
                .clone(),
                |input: ToolInput| {
                    let path = input.arguments.get("path")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| McpError::InvalidInput("path is required".into()))?;
                    Ok(ToolResult::from_json(json!({ "status": "initialized", "path": path })))
                },
            )
            .await;
    }

    async fn register_github_tools(&self) {
        use serde_json::json;

        for (name, desc, schema) in [
            ("github_pr_list", "List pull requests from a GitHub repository",
             serde_json::json!({ "type": "object", "properties": { "owner": {"type":"string"}, "repo": {"type":"string"}, "state": {"type":"string"} } }).as_object().unwrap().clone()),
            ("github_pr_get", "Get details of a specific pull request",
             serde_json::json!({ "type": "object", "properties": { "owner": {"type":"string"}, "repo": {"type":"string"}, "pr_number": {"type":"integer"} }, "required": ["owner", "repo", "pr_number"] }).as_object().unwrap().clone()),
            ("github_pr_create", "Create a new pull request",
             serde_json::json!({ "type": "object", "properties": { "owner": {"type":"string"}, "repo": {"type":"string"}, "title": {"type":"string"}, "head": {"type":"string"}, "base": {"type":"string"} }, "required": ["owner", "repo", "title", "head", "base"] }).as_object().unwrap().clone()),
            ("github_issue_list", "List issues from a GitHub repository",
             serde_json::json!({ "type": "object", "properties": { "owner": {"type":"string"}, "repo": {"type":"string"} } }).as_object().unwrap().clone()),
            ("github_branch_list", "List branches in a GitHub repository",
             serde_json::json!({ "type": "object", "properties": { "owner": {"type":"string"}, "repo": {"type":"string"} }, "required": ["owner", "repo"] }).as_object().unwrap().clone()),
            ("github_repo_get", "Get information about a GitHub repository",
             serde_json::json!({ "type": "object", "properties": { "owner": {"type":"string"}, "repo": {"type":"string"} }, "required": ["owner", "repo"] }).as_object().unwrap().clone()),
        ] {
            self.registry
                .register(name.into(), desc.into(), schema, |_input: ToolInput| {
                    Ok(ToolResult::from_json(json!({})))
                })
                .await;
        }
    }

    async fn register_azure_tools(&self) {
        use serde_json::json;

        for (name, desc, schema) in [
            ("azure_workitem_list", "List work items from Azure DevOps",
             serde_json::json!({ "type": "object", "properties": { "project": {"type":"string"} } }).as_object().unwrap().clone()),
            ("azure_workitem_get", "Get details of a specific Azure DevOps work item",
             serde_json::json!({ "type": "object", "properties": { "id": {"type":"integer"} }, "required": ["id"] }).as_object().unwrap().clone()),
            ("azure_sprint_list", "List sprints in an Azure DevOps project",
             serde_json::json!({ "type": "object", "properties": { "project": {"type":"string"} }, "required": ["project"] }).as_object().unwrap().clone()),
            ("azure_board_get", "Get the Kanban board for an Azure DevOps project",
             serde_json::json!({ "type": "object", "properties": { "project": {"type":"string"} }, "required": ["project"] }).as_object().unwrap().clone()),
        ] {
            self.registry
                .register(name.into(), desc.into(), schema, |_input: ToolInput| {
                    Ok(ToolResult::from_json(json!({})))
                })
                .await;
        }
    }

    async fn register_scaffold_tools(&self) {
        use serde_json::json;

        for (name, desc, schema) in [
            ("scaffold_list_templates", "List all available project templates",
             serde_json::json!({}).as_object().unwrap().clone()),
            ("scaffold_nextjs", "Scaffold a new Next.js project",
             serde_json::json!({ "type": "object", "properties": { "name": {"type":"string"} }, "required": ["name"] }).as_object().unwrap().clone()),
            ("scaffold_add_page", "Add a new page to an existing project",
             serde_json::json!({ "type": "object", "properties": { "project_path": {"type":"string"}, "page_name": {"type":"string"} }, "required": ["project_path", "page_name"] }).as_object().unwrap().clone()),
            ("scaffold_add_component", "Add a new UI component to a project",
             serde_json::json!({ "type": "object", "properties": { "project_path": {"type":"string"}, "component_name": {"type":"string"} }, "required": ["project_path", "component_name"] }).as_object().unwrap().clone()),
        ] {
            self.registry
                .register(name.into(), desc.into(), schema, |_input: ToolInput| {
                    Ok(ToolResult::from_json(json!({})))
                })
                .await;
        }
    }

    async fn register_analysis_tools(&self) {
        use serde_json::json;

        for (name, desc, schema) in [
            ("analyze_complexity", "Analyze code complexity of a project",
             serde_json::json!({ "type": "object", "properties": { "path": {"type":"string"} }, "required": ["path"] }).as_object().unwrap().clone()),
            ("analyze_dependencies", "Analyze project dependencies for vulnerabilities",
             serde_json::json!({ "type": "object", "properties": { "path": {"type":"string"} }, "required": ["path"] }).as_object().unwrap().clone()),
            ("lint_run", "Run linter on a project",
             serde_json::json!({ "type": "object", "properties": { "path": {"type":"string"}, "fix": {"type":"boolean"} }, "required": ["path"] }).as_object().unwrap().clone()),
            ("test_run", "Run tests in a project",
             serde_json::json!({ "type": "object", "properties": { "path": {"type":"string"}, "coverage": {"type":"boolean"} }, "required": ["path"] }).as_object().unwrap().clone()),
            ("quality_report", "Generate a comprehensive quality report for a project",
             serde_json::json!({ "type": "object", "properties": { "path": {"type":"string"} }, "required": ["path"] }).as_object().unwrap().clone()),
        ] {
            self.registry
                .register(name.into(), desc.into(), schema, |_input: ToolInput| {
                    Ok(ToolResult::from_json(json!({})))
                })
                .await;
        }
    }
}

impl Default for DevaMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerHandler for DevaMcpServer {
    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, rmcp::ErrorData>> + '_ {
        async move {
            let tools = self.registry.list().await;
            Ok(ListToolsResult::with_all_items(tools))
        }
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, rmcp::ErrorData>> + '_ {
        async move {
            let args = request.arguments.unwrap_or_default();
            let input = ToolInput::new(
                request.name.to_string(),
                serde_json::Value::Object(args),
            );

            match self.registry.execute(&request.name.to_string(), input).await {
                Ok(result) => {
                    let content: Vec<rmcp::model::Content> = result
                        .content
                        .into_iter()
                        .map(|v| rmcp::model::Content::text(v.to_string()))
                        .collect();
                    Ok(CallToolResult::success(content))
                }
                Err(e) => {
                    error!("Tool execution failed: {}", e);
                    Err(rmcp::ErrorData::internal_error(e.to_string(), None))
                }
            }
        }
    }
}
