use crate::character_model::CharacterData;
use crate::pdf_filler::PdfFiller;
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

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
    #[allow(dead_code)]
    pdf_filler: PdfFiller,
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            pdf_filler: PdfFiller::new(false),
        }
    }

    pub fn run(&self) -> io::Result<()> {
        log_to_file("MCP Server starting up");

        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let reader = BufReader::new(stdin);

        log_to_file("Entering main loop");

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                continue;
            }

            log_to_file(&format!("Processing line: {}", trimmed));

            if let Some(response) = self.handle_message(trimmed) {
                let response_str = serde_json::to_string(&response).unwrap();
                log_to_file(&format!("Sending response: {}", response_str));

                stdout.write_all(response_str.as_bytes())?;
                stdout.write_all(b"\n")?;
                stdout.flush()?;
                
                log_to_file("Response sent successfully");
            }
        }

        log_to_file("MCP Server shutting down cleanly");
        Ok(())
    }

    fn handle_message(&self, message: &str) -> Option<Value> {
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
            Some("tools/call") => Some(self.handle_tools_call(id, &request)),
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
                                "return_pdf_content": {
                                    "type": "boolean",
                                    "default": false,
                                    "description": "Return the PDF file content as base64 for the LLM to save (WARNING: Large response)"
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

    fn handle_tools_call(&self, id: Option<Value>, request: &Value) -> Value {
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
            "fill_dnd_character_sheet" => self.handle_fill_character_sheet(id, arguments),
            _ => self.error_response(id, -32602, "Unknown tool", None),
        }
    }

    fn handle_fill_character_sheet(&self, id: Option<Value>, arguments: &Value) -> Value {
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

        // Get return PDF content setting
        let return_pdf_content = arguments
            .get("return_pdf_content")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Get rule violation setting
        let allow_violations = arguments
            .get("allow_rule_violations")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        log_to_file(&format!(
            "Processing PDF: output_path={}, allow_violations={}, return_content={}",
            output_path, allow_violations, return_pdf_content
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
                    "calculated_fields": result.calculated_fields,
                    "message": format!("PDF character sheet created successfully at: {}", result.output_file)
                });

                // Include PDF content if requested (WARNING: Large response)
                if return_pdf_content {
                    match fs::read(&result.output_file) {
                        Ok(pdf_bytes) => {
                            // Check size limit (5MB for MCP response)
                            if pdf_bytes.len() > 5 * 1024 * 1024 {
                                log_to_file("PDF too large for content return");
                                response_data["pdf_content_error"] = json!("PDF file too large (>5MB) for MCP response. File saved locally.");
                                response_data["file_size"] = json!(pdf_bytes.len());
                            } else {
                                let base64_content = general_purpose::STANDARD.encode(&pdf_bytes);
                                response_data["pdf_content"] = json!(base64_content);
                                response_data["file_size"] = json!(pdf_bytes.len());
                                log_to_file(&format!("PDF content encoded as base64, size: {} bytes", pdf_bytes.len()));
                            }
                        }
                        Err(e) => {
                            log_to_file(&format!("Failed to read PDF file: {}", e));
                            response_data["pdf_content_error"] = json!(e.to_string());
                        }
                    }
                }

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
                let final_response = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": format!("Character sheet created successfully at: {}", result.output_file)
                            }
                        ],
                        "isError": false
                    }
                });
                
                log_to_file(&format!("Response structure: {}", 
                    serde_json::to_string_pretty(&final_response)
                        .unwrap_or_else(|_| "Failed to serialize".to_string())
                ));
                
                final_response
            }
            Err(e) => {
                log_to_file(&format!("PDF processing failed: {}", e));
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": format!("Error: PDF processing failed - {}", e)
                            }
                        ],
                        "isError": true
                    }
                })
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
