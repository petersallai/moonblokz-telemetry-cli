use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub enum Command {
    SetUpdateInterval {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        active_period: u64,
        inactive_period: u64,
    },
    SetLogLevel {
        node_id: Option<u32>,
        log_level: String,
    },
    SetLogFilter {
        node_id: Option<u32>,
        log_filter: String,
    },
    Command {
        node_id: Option<u32>,
        command: String,
    },
    UpdateNode {
        node_id: Option<u32>,
    },
    UpdateProbe {
        node_id: Option<u32>,
    },
    RebootProbe {
        node_id: Option<u32>,
    },
    StartMeasurement {
        node_id: u32,
        sequence: u32,
    },
    Quit,
}

impl Command {
    pub fn to_json(&self) -> Result<Value> {
        match self {
            Command::SetUpdateInterval {
                start_time,
                end_time,
                active_period,
                inactive_period,
            } => {
                let params = json!({
                    "start_time": start_time.to_rfc3339(),
                    "end_time": end_time.to_rfc3339(),
                    "active_period": active_period,
                    "inactive_period": inactive_period,
                });

                Ok(json!({
                    "command": "set_update_interval",
                    "parameters": params,
                }))
            }
            Command::SetLogLevel { node_id, log_level } => {
                let mut params = json!({
                    "log_level": log_level,
                });

                if let Some(id) = node_id {
                    params["node id"] = json!(id);
                }

                Ok(json!({
                    "command": "set_log_level",
                    "parameters": params,
                }))
            }
            Command::SetLogFilter { node_id, log_filter } => {
                let mut params = json!({
                    "log_filter": log_filter,
                });

                if let Some(id) = node_id {
                    params["node id"] = json!(id);
                }

                Ok(json!({
                    "command": "set_log_filter",
                    "parameters": params,
                }))
            }
            Command::Command { node_id, command } => {
                let mut params = json!({
                    "command": command,
                });

                if let Some(id) = node_id {
                    params["node id"] = json!(id);
                }

                Ok(json!({
                    "command": "command",
                    "parameters": params,
                }))
            }
            Command::UpdateNode { node_id } => {
                let mut params = json!({});

                if let Some(id) = node_id {
                    params["node id"] = json!(id);
                }

                Ok(json!({
                    "command": "update_node",
                    "parameters": params,
                }))
            }
            Command::UpdateProbe { node_id } => {
                let mut params = json!({});

                if let Some(id) = node_id {
                    params["node id"] = json!(id);
                }

                Ok(json!({
                    "command": "update_probe",
                    "parameters": params,
                }))
            }
            Command::RebootProbe { node_id } => {
                let mut params = json!({});

                if let Some(id) = node_id {
                    params["node id"] = json!(id);
                }

                Ok(json!({
                    "command": "reboot_probe",
                    "parameters": params,
                }))
            }
            Command::StartMeasurement { node_id, sequence } => {
                let params = json!({
                    "node id": node_id,
                    "sequence": sequence,
                });

                Ok(json!({
                    "command": "start_measurement",
                    "parameters": params,
                }))
            }
            Command::Quit => Err(anyhow!("Quit command cannot be converted to JSON")),
        }
    }
}

pub fn parse_command(input: &str) -> Result<Command> {
    let input = input.trim();

    // Check for quit commands
    let lower = input.to_lowercase();
    if lower == "quit" || lower == "exit" || lower == "bye" {
        return Ok(Command::Quit);
    }

    // Find command name and parameters
    let (cmd_name, params_str) = if let Some(paren_idx) = input.find('(') {
        if !input.ends_with(')') {
            return Err(anyhow!("Missing closing parenthesis"));
        }
        let cmd = input[..paren_idx].trim();
        let params = &input[paren_idx + 1..input.len() - 1];
        (cmd, Some(params))
    } else {
        (input, None)
    };

    let cmd_lower = cmd_name.to_lowercase();

    match cmd_lower.as_str() {
        "set_update_interval" => {
            let params = params_str.ok_or_else(|| anyhow!("set_update_interval requires parameters"))?;
            parse_set_update_interval(params)
        }
        "set_log_level" => {
            let params = params_str.ok_or_else(|| anyhow!("set_log_level requires parameters"))?;
            parse_set_log_level(params)
        }
        "set_log_filter" => {
            let params = params_str.ok_or_else(|| anyhow!("set_log_filter requires parameters"))?;
            parse_set_log_filter(params)
        }
        "command" => {
            let params = params_str.ok_or_else(|| anyhow!("command requires parameters"))?;
            parse_command_cmd(params)
        }
        "update_node" => {
            let params = params_str.unwrap_or("");
            parse_update_node(params)
        }
        "update_probe" => {
            let params = params_str.unwrap_or("");
            parse_update_probe(params)
        }
        "reboot_probe" => {
            let params = params_str.unwrap_or("");
            parse_reboot_probe(params)
        }
        "start_measurement" => {
            let params = params_str.ok_or_else(|| anyhow!("start_measurement requires parameters"))?;
            parse_start_measurement(params)
        }
        _ => Err(anyhow!("Unknown command: {}", cmd_name)),
    }
}

fn parse_params(params_str: &str) -> Result<Vec<(String, String)>> {
    let mut params = Vec::new();
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut in_value = false;
    let mut in_quotes = false;
    let mut chars = params_str.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_value {
            if ch == '"' {
                in_quotes = !in_quotes;
                current_value.push(ch);
            } else if ch == ',' && !in_quotes {
                params.push((current_key.trim().to_string(), current_value.trim().to_string()));
                current_key.clear();
                current_value.clear();
                in_value = false;
            } else {
                current_value.push(ch);
            }
        } else {
            if ch == '=' {
                in_value = true;
            } else if !ch.is_whitespace() || !current_key.is_empty() {
                current_key.push(ch);
            }
        }
    }

    if !current_key.is_empty() || !current_value.is_empty() {
        params.push((current_key.trim().to_string(), current_value.trim().to_string()));
    }

    Ok(params)
}

fn get_param<'a>(params: &'a [(String, String)], key: &str) -> Option<&'a str> {
    params.iter().find(|(k, _)| k.eq_ignore_ascii_case(key)).map(|(_, v)| v.as_str())
}

fn parse_node_id(params: &[(String, String)]) -> Result<Option<u32>> {
    if let Some(value) = get_param(params, "node_id") {
        let id = value.parse::<u32>().map_err(|_| anyhow!("Invalid node_id: must be a positive integer"))?;
        Ok(Some(id))
    } else {
        Ok(None)
    }
}

fn parse_iso_timestamp(s: &str) -> Result<DateTime<Utc>> {
    // Try parsing with timezone first
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try parsing ISO 8601 with offset
    if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%z") {
        return Ok(dt.with_timezone(&Utc));
    }

    if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M%z") {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try with explicit +/- offset format
    if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%:z") {
        return Ok(dt.with_timezone(&Utc));
    }

    Err(anyhow!("Invalid ISO 8601 timestamp: {}", s))
}

fn parse_set_update_interval(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;

    // Reject node_id parameter - this command targets all probes
    if get_param(&params, "node_id").is_some() {
        return Err(anyhow!("set_update_interval does not accept node_id parameter"));
    }

    let start_time_str = get_param(&params, "start_time").ok_or_else(|| anyhow!("Missing start_time parameter"))?;
    let end_time_str = get_param(&params, "end_time").ok_or_else(|| anyhow!("Missing end_time parameter"))?;
    let active_period_str = get_param(&params, "active_period").ok_or_else(|| anyhow!("Missing active_period parameter"))?;
    let inactive_period_str = get_param(&params, "inactive_period").ok_or_else(|| anyhow!("Missing inactive_period parameter"))?;

    let start_time = parse_iso_timestamp(start_time_str)?;
    let end_time = parse_iso_timestamp(end_time_str)?;
    let active_period = active_period_str
        .parse::<u64>()
        .map_err(|_| anyhow!("Invalid active_period: must be a positive integer"))?;
    let inactive_period = inactive_period_str
        .parse::<u64>()
        .map_err(|_| anyhow!("Invalid inactive_period: must be a positive integer"))?;

    Ok(Command::SetUpdateInterval {
        start_time,
        end_time,
        active_period,
        inactive_period,
    })
}

fn parse_set_log_level(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;
    let node_id = parse_node_id(&params)?;

    let log_level = get_param(&params, "log_level")
        .ok_or_else(|| anyhow!("Missing log_level parameter"))?
        .to_uppercase();

    // Validate log level
    match log_level.as_str() {
        "TRACE" | "DEBUG" | "INFO" | "WARN" | "ERROR" => {}
        _ => return Err(anyhow!("Invalid log_level: must be TRACE, DEBUG, INFO, WARN, or ERROR")),
    }

    Ok(Command::SetLogLevel { node_id, log_level })
}

fn parse_set_log_filter(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;
    let node_id = parse_node_id(&params)?;

    let log_filter = get_param(&params, "log_filter")
        .ok_or_else(|| anyhow!("Missing log_filter parameter"))?
        .to_string();

    Ok(Command::SetLogFilter { node_id, log_filter })
}

fn parse_command_cmd(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;
    let node_id = parse_node_id(&params)?;

    let command = get_param(&params, "command").ok_or_else(|| anyhow!("Missing command parameter"))?.to_string();

    Ok(Command::Command { node_id, command })
}

fn parse_update_node(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;
    let node_id = parse_node_id(&params)?;

    Ok(Command::UpdateNode { node_id })
}

fn parse_update_probe(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;
    let node_id = parse_node_id(&params)?;

    Ok(Command::UpdateProbe { node_id })
}

fn parse_reboot_probe(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;
    let node_id = parse_node_id(&params)?;

    Ok(Command::RebootProbe { node_id })
}

fn parse_start_measurement(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;

    let node_id = parse_node_id(&params)?.ok_or_else(|| anyhow!("node_id is required for start_measurement command"))?;

    let sequence_str = get_param(&params, "sequence").ok_or_else(|| anyhow!("Missing sequence parameter"))?;

    let sequence = sequence_str
        .parse::<u32>()
        .map_err(|_| anyhow!("Invalid sequence: must be a positive integer"))?;

    Ok(Command::StartMeasurement { node_id, sequence })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_quit_commands() {
        assert!(matches!(parse_command("quit").unwrap(), Command::Quit));
        assert!(matches!(parse_command("exit").unwrap(), Command::Quit));
        assert!(matches!(parse_command("bye").unwrap(), Command::Quit));
        assert!(matches!(parse_command("QUIT").unwrap(), Command::Quit));
    }

    #[test]
    fn test_parse_set_log_level() {
        let cmd = parse_command("set_log_level(node_id=21, log_level=DEBUG)").unwrap();
        match cmd {
            Command::SetLogLevel { node_id, log_level } => {
                assert_eq!(node_id, Some(21));
                assert_eq!(log_level, "DEBUG");
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_parse_update_node_with_node_id() {
        let cmd = parse_command("update_node(node_id=42)").unwrap();
        match cmd {
            Command::UpdateNode { node_id } => {
                assert_eq!(node_id, Some(42));
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_parse_update_node_without_node_id() {
        let cmd = parse_command("update_node()").unwrap();
        match cmd {
            Command::UpdateNode { node_id } => {
                assert_eq!(node_id, None);
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_parse_start_measurement() {
        let cmd = parse_command("start_measurement(node_id=21, sequence=42)").unwrap();
        match cmd {
            Command::StartMeasurement { node_id, sequence } => {
                assert_eq!(node_id, 21);
                assert_eq!(sequence, 42);
            }
            _ => panic!("Wrong command type"),
        }
    }

    #[test]
    fn test_parse_start_measurement_requires_node_id() {
        let result = parse_command("start_measurement(sequence=1)");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("node_id is required"));
    }
}
