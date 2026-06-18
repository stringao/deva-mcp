use mockito::{Server, Mock};
use deva_mcp::github_executor::GitHubToolExecutor;
use deva_mcp::tools::registry::ToolInput;
use serde_json::json;

/// Helper to create ToolInput
fn tool_input(name: &str, args: serde_json::Value) -> ToolInput {
    ToolInput::new(name.to_string(), args)
}

#[tokio::test]
async fn test_github_pr_list_success() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("GET", "/repos/stringao/deva/pulls")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[
            {"number": 1, "title": "Test PR", "state": "open", "user": {"login": "testuser"}}
        ]"#)
        .create();

    let executor = GitHubToolExecutor::new(
        deva_github::GitHubClient::new("token", "stringao", "deva")
    );

    let result = executor.list_prs_async(None).await;

    assert!(result.is_ok());
    let tool_result = result.unwrap();
    let content = &tool_result.content[0];
    assert!(content.get("pull_requests").is_some());
}

#[tokio::test]
async fn test_github_pr_get_success() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("GET", "/repos/stringao/deva/pulls/1")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"number": 1, "title": "Test PR", "state": "open"}"#)
        .create();

    let executor = GitHubToolExecutor::new(
        deva_github::GitHubClient::new("token", "stringao", "deva")
    );

    let result = executor.get_pr_async(1).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_github_pr_create_success() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("POST", "/repos/stringao/deva/pulls")
        .match_body(mockito::Matcher::Json(json!({
            "title": "New PR",
            "head": "feature-branch",
            "base": "main"
        })))
        .with_status(201)
        .with_header("content-type", "application/json")
        .with_body(r#"{"number": 42, "title": "New PR", "state": "open"}"#)
        .create();

    let executor = GitHubToolExecutor::new(
        deva_github::GitHubClient::new("token", "stringao", "deva")
    );

    let result = executor.create_pr_async("New PR", "feature-branch", "main", None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_github_issue_list_success() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("GET", "/repos/stringao/deva/issues")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[
            {"number": 1, "title": "Bug fix", "state": "open"}
        ]"#)
        .create();

    let executor = GitHubToolExecutor::new(
        deva_github::GitHubClient::new("token", "stringao", "deva")
    );

    let result = executor.list_issues_async(Some("open")).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_github_branch_list_success() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("GET", "/repos/stringao/deva/branches")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[
            {"name": "main"},
            {"name": "feature/test"}
        ]"#)
        .create();

    let executor = GitHubToolExecutor::new(
        deva_github::GitHubClient::new("token", "stringao", "deva")
    );

    let result = executor.list_branches_async().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_github_repo_get_success() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("GET", "/repos/stringao/deva/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "name": "deva",
            "full_name": "stringao/deva",
            "description": "Deva project",
            "private": false
        }"#)
        .create();

    let executor = GitHubToolExecutor::new(
        deva_github::GitHubClient::new("token", "stringao", "deva")
    );

    let result = executor.get_repo_async().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_github_pr_not_found() {
    let mut server = Server::new_async().await;
    let _mock = server.mock("GET", "/repos/stringao/deva/pulls/999")
        .with_status(404)
        .with_body(r#"{"message": "Not Found"}"#)
        .create();

    let executor = GitHubToolExecutor::new(
        deva_github::GitHubClient::new("token", "stringao", "deva")
    );

    let result = executor.get_pr_async(999).await;

    // Should error
    assert!(result.is_err());
}