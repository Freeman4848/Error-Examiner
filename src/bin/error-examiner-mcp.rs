#![allow(dead_code)]

#[path = "../mcp_audit.rs"]
mod mcp_audit;
#[path = "../mcp_server.rs"]
mod mcp_server;
#[path = "../parser_registry.rs"]
mod parser_registry;
#[path = "../parser_schema.rs"]
mod parser_schema;
#[path = "../parser_text.rs"]
mod parser_text;
#[path = "../preprocess.rs"]
mod preprocess;
#[path = "../preprocess_raw.rs"]
mod preprocess_raw;
#[path = "../schema_management.rs"]
mod schema_management;
#[path = "../storage.rs"]
mod storage;

use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = mcp_server::ErrorExaminerMcp
        .serve(rmcp::transport::stdio())
        .await?;
    server.waiting().await?;
    Ok(())
}
