mod skills;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use skills::{Skill, load_skills};
use std::env;
use std::process::Stdio;
use log::{info, error, warn};
use env_logger::Env;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger to write to stderr (default), but strictly ensure it doesn't touch stdout
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stderr)
        .init();

    info!("Starting Rust Agentic Skills MCP Server...");

    // Load Skills dynamically
    let current_dir = env::current_dir()?;
    let skills = load_skills(&current_dir).unwrap_or_else(|e| {
        warn!("Failed to load skills: {}", e);
        Vec::new()
    });
    info!(
        "Loaded {} skills: {:?}",
        skills.len(),
        skills.iter().map(|s| &s.name).collect::<Vec<_>>()
    );

    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut stdout = tokio::io::stdout();

    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to parse request: {} | Line: {}", e, line);
                continue;
            }
        };

        let response = handle_request(req, &skills).await;

        if let Some(resp) = response {
            let mut resp_str = serde_json::to_string(&resp)?;
            resp_str.push('\n');
            stdout.write_all(resp_str.as_bytes()).await?;
            stdout.flush().await?;
        }
    }

    Ok(())
}

async fn handle_request(req: JsonRpcRequest, skills: &[Skill]) -> Option<JsonRpcResponse> {
    let method = req.method.as_str();
    let id = req.id.clone();

    let result = match method {
        "initialize" => {
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "listChanged": false }
                },
                "serverInfo": {
                    "name": "rust-agentic-skills",
                    "version": "1.0.0"
                }
            })
        }
        "notifications/initialized" => {
            return None;
        }
        "tools/list" => {
            let tools: Vec<Value> = skills.iter().map(|s| {
                json!({
                    "name": s.tool_name(),
                    "description": s.tool_description(),
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "args": { "type": "string", "description": "Arguments for the skill script." }
                        },
                        "required": ["args"]
                    }
                })
            }).collect();

            json!({ "tools": tools })
        }
        "tools/call" => handle_tool_call(req.params, skills).await,
        "ping" => json!({}),
        _ => {
            warn!("Unknown method: {}", method);
            return Some(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(json!({"code": -32601, "message": "Method not found"})),
            });
        }
    };

    Some(JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(result),
        error: None,
    })
}

async fn handle_tool_call(params: Option<Value>, skills: &[Skill]) -> Value {
    let params = params.unwrap_or(json!({}));
    let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
    let args_val = params.get("arguments").cloned().unwrap_or(json!({}));
    let args_str = args_val.get("args").and_then(|a| a.as_str()).unwrap_or("");

    if let Some(skill) = skills.iter().find(|s| s.tool_name() == name) {
        // Look for scripts
        let scripts_dir = skill.path.join("scripts");
        if scripts_dir.exists() {
            // Heuristic: Try to find a logical script.
            // For now, we just list files and try to run specific known ones or just return info.
            // For issue fix completeness, we will try to run `explain_error.sh` if it is Lint Hunter
            if name == "lint_hunter" {
                let script_path = scripts_dir.join("explain_error.sh");
                if script_path.exists() {
                    let output = Command::new("sh")
                        .arg(&script_path)
                        .arg(args_str)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .stdin(Stdio::null())
                        .output()
                        .await;

                    match output {
                        Ok(o) => {
                            let stdout = String::from_utf8_lossy(&o.stdout);
                            let stderr = String::from_utf8_lossy(&o.stderr);
                            return json!({ "content": [{ "type": "text", "text": format!("{}\n{}", stdout, stderr) }] });
                        }
                        Err(e) => {
                            return json!({ "content": [{ "type": "text", "text": format!("Failed to execute script: {}", e) }], "isError": true });
                        }
                    }
                }
            }
        }

        // Default response for skills without executable scripts in this MVP
        json!({
            "content": [
                {
                    "type": "text",
                    "text": format!("Skill '{}' activated. (Executable logic pending for generic scripts). Paths: {:?}", name, skill.path)
                }
            ]
        })
    } else {
        json!({
            "content": [{ "type": "text", "text": format!("Tool not found: {}", name) }],
            "isError": true
        })
    }
}
