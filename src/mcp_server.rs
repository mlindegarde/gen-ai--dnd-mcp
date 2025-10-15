use crate::character_model::CharacterData;
use crate::pdf_filler::PdfFiller;
use serde_json::{json, Value};
use std::fs::OpenOptions;
use std::io::Write;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

fn log_to_file(message: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/Users/lindegar/learningplace/specify--test/mcp_debug.log")
    {
        let _ = writeln!(
            file,
            "[{}] {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            message
        );
    }
}

pub struct McpServer {
    pdf_filler: PdfFiller,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            pdf_filler: PdfFiller::new(false),
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        log_to_file("MCP Server starting up");

        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        log_to_file("Entering main loop");

        loop {
            line.clear();
            log_to_file("Waiting for next message...");
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    log_to_file("EOF received, breaking");
                    break;
                } // EOF
                Ok(bytes_read) => {
                    log_to_file(&format!("Read {} bytes", bytes_read));
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        log_to_file("Empty line, continuing");
                        continue;
                    }

                    log_to_file(&format!("Processing line: {}", trimmed));

                    if let Some(response) = self.handle_message(trimmed).await {
                        let response_str = serde_json::to_string(&response).unwrap();
                        log_to_file(&format!("Sending response: {}", response_str));

                        if let Err(e) = stdout.write_all(response_str.as_bytes()).await {
                            log_to_file(&format!("Error writing response: {}", e));
                            break;
                        }
                        if let Err(e) = stdout.write_all(b"\n").await {
                            log_to_file(&format!("Error writing newline: {}", e));
                            break;
                        }
                        if let Err(e) = stdout.flush().await {
                            log_to_file(&format!("Error flushing stdout: {}", e));
                            break;
                        }
                        log_to_file("Response sent successfully, continuing loop");
                    } else {
                        log_to_file("No response to send, continuing loop");
                    }
                }
                Err(e) => {
                    log_to_file(&format!("Error reading: {}", e));
                    break;
                }
            }
        }

        log_to_file("MCP Server shutting down");
        Ok(())
    }

    async fn handle_message(&self, message: &str) -> Option<Value> {
        log_to_file(&format!(
            "=== RAW MESSAGE ===\n{}\n=== END RAW MESSAGE ===",
            message
        ));

        let request: Value = match serde_json::from_str(message) {
            Ok(req) => {
                log_to_file(&format!(
                    "Parsed request: {}",
                    serde_json::to_string_pretty(&req)
                        .unwrap_or_else(|_| "Failed to pretty print".to_string())
                ));
                req
            }
            Err(e) => {
                log_to_file(&format!("Parse error: {}", e));
                return Some(self.error_response(None, -32700, "Parse error", None));
            }
        };

        let id = request.get("id").cloned();
        let method = request.get("method").and_then(|m| m.as_str());

        log_to_file(&format!("Extracted - Method: {:?}", method));
        log_to_file(&format!("Extracted - ID: {:?}", id));
        log_to_file(&format!(
            "ID as JSON: {}",
            serde_json::to_string(&id).unwrap_or_else(|_| "null".to_string())
        ));

        match method {
            Some("initialize") => Some(self.handle_initialize(id)),
            Some("initialized") | Some("notifications/initialized") => None, // No response needed for initialized notification
            Some("tools/list") => Some(self.handle_tools_list(id)),
            Some("tools/call") => Some(self.handle_tools_call(id, &request).await),
            Some(unknown_method) => {
                log_to_file(&format!("Unknown method: {}", unknown_method));
                log_to_file(&format!("Returning error with ID: {:?}", id));
                Some(self.error_response(id, -32601, "Method not found", None))
            }
            None => {
                log_to_file("No method found in request");
                log_to_file(&format!("Returning error with ID: {:?}", id));
                Some(self.error_response(id, -32600, "Invalid Request", None))
            }
        }
    }

    fn handle_initialize(&self, id: Option<Value>) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "dnd-character-sheet-filler",
                    "version": "1.0.0"
                }
            }
        })
    }

    fn handle_tools_list(&self, id: Option<Value>) -> Value {
        json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {
                        "name": "fill_dnd_character_sheet",
                        "description": "Fills a D&D 5e character sheet PDF with provided character data including spells, proficiencies, and narrative elements",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "character_data": {
                                    "type": "object",
                                    "description": "Complete D&D 5e character information"
                                },
                                "output_path": {
                                    "type": "string",
                                    "description": "Path where filled PDF should be saved",
                                    "default": "filled_character_sheet.pdf"
                                },
                                "allow_rule_violations": {
                                    "type": "boolean",
                                    "default": false,
                                    "description": "Allow character data that violates D&D 5e rules"
                                }
                            },
                            "required": ["character_data"]
                        }
                    }
                ]
            }
        })
    }

    async fn handle_tools_call(&self, id: Option<Value>, request: &Value) -> Value {
        let params = match request.get("params") {
            Some(p) => p,
            None => return self.error_response(id, -32602, "Invalid params", None),
        };

        let tool_name = match params.get("name").and_then(|n| n.as_str()) {
            Some("fill_dnd_character_sheet") => "fill_dnd_character_sheet",
            _ => return self.error_response(id, -32602, "Unknown tool", None),
        };

        let arguments = match params.get("arguments") {
            Some(args) => args,
            None => return self.error_response(id, -32602, "Missing arguments", None),
        };

        match tool_name {
            "fill_dnd_character_sheet" => self.handle_fill_character_sheet(id, arguments).await,
            _ => self.error_response(id, -32602, "Unknown tool", None),
        }
    }

    async fn handle_fill_character_sheet(&self, id: Option<Value>, arguments: &Value) -> Value {
        log_to_file("Starting fill_character_sheet processing");

        // Parse character data
        let character_data = match arguments.get("character_data") {
            Some(data) => match serde_json::from_value::<CharacterData>(data.clone()) {
                Ok(char_data) => {
                    log_to_file("Character data parsed successfully");
                    char_data
                }
                Err(e) => {
                    log_to_file(&format!("Character data parse error: {}", e));
                    return self.error_response(
                        id,
                        -32602,
                        "Invalid character data",
                        Some(json!({"parse_error": e.to_string()})),
                    );
                }
            },
            None => {
                log_to_file("Missing character_data parameter");
                return self.error_response(id, -32602, "Missing character_data", None);
            }
        };

        // Get output path
        let output_path = arguments
            .get("output_path")
            .and_then(|p| p.as_str())
            .unwrap_or("filled_character_sheet.pdf");

        // Get rule violation setting
        let allow_violations = arguments
            .get("allow_rule_violations")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        log_to_file(&format!(
            "Processing PDF: output_path={}, allow_violations={}",
            output_path, allow_violations
        ));

        // Create PDF filler with appropriate settings
        let filler = PdfFiller::new(allow_violations);

        // Fill the character sheet
        match filler.fill_character_sheet(
            &character_data,
            "docs/5E_CharacterSheet_Fillable.pdf",
            output_path,
        ) {
            Ok(result) => {
                log_to_file("PDF processing completed successfully");
                let mut response_data = json!({
                    "success": true,
                    "output_file": result.output_file,
                    "calculated_fields": result.calculated_fields
                });

                if !result.validation_errors.is_empty() {
                    response_data["validation_errors"] = json!(result
                        .validation_errors
                        .iter()
                        .map(|e| json!({
                            "error": e.to_string(),
                            "severity": if allow_violations { "warning" } else { "error" }
                        }))
                        .collect::<Vec<_>>());

                    if allow_violations {
                        response_data["rule_violations_overridden"] = json!(true);
                    }
                }

                log_to_file("Returning success response");
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": response_data
                })
            }
            Err(e) => {
                log_to_file(&format!("PDF processing failed: {}", e));
                self.error_response(
                    id,
                    -32603,
                    "PDF processing failed",
                    Some(json!({"error": e.to_string()})),
                )
            }
        }
    }

    fn error_response(
        &self,
        id: Option<Value>,
        code: i32,
        message: &str,
        data: Option<Value>,
    ) -> Value {
        let mut error = json!({
            "code": code,
            "message": message
        });

        if let Some(data) = data {
            error["data"] = data;
        }

        let response = json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": error
        });

        log_to_file(&format!(
            "Sending error response: {}",
            serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize".to_string())
        ));
        response
    }
}
