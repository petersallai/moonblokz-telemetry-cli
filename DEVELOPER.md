# Developer Documentation

## Architecture

The MoonBlokz Telemetry CLI is built in Rust using Tokio for async runtime and follows a modular architecture:

### Module Structure

```
src/
├── main.rs       - Entry point, CLI argument parsing, REPL implementation
├── config.rs     - Configuration loading from TOML
├── parser.rs     - Command grammar parser
└── client.rs     - HTTP client for hub communication
```

### Key Components

#### 1. Configuration Module (`config.rs`)

- Loads configuration from `config.toml` using the `toml` crate
- Supports custom config paths via `--config` flag
- Required fields:
  - `api-key`: Authentication token for the hub
  - `hub-url`: Base URL of the telemetry hub

#### 2. Parser Module (`parser.rs`)

Implements the command grammar parser with the following features:

- **Case-insensitive** command parsing
- **Flexible parameter parsing** with support for:
  - Integer node IDs
  - ISO 8601 timestamps with timezone conversion to UTC
  - String values (with or without quotes)
  - Enumerated values (log levels)
- **Command variants**:
  - `SetUpdateInterval` - Scheduling parameters
  - `SetLogLevel` - Verbosity control
  - `SetLogFilter` - Filter string updates
  - `Command` - Raw USB commands
  - `UpdateNode` - Firmware updates for RP2040
  - `UpdateProbe` - Probe self-updates
  - `RebootProbe` - Raspberry Pi reboot
  - `Quit` - Exit interactive mode

Each command converts to JSON format matching the hub's API specification.

#### 3. Client Module (`client.rs`)

- Uses `reqwest` for HTTP/HTTPS communication
- Sends POST requests to `/command` endpoint
- Handles HTTP status codes:
  - `200 OK` → Success
  - `401 Unauthorized` → Invalid API key
  - `4xx` → Client errors
  - `5xx` → Server errors
- 30-second timeout for requests

#### 4. Main Module (`main.rs`)

- CLI argument parsing with `clap`
- Two modes of operation:
  1. **Single command mode**: Execute one command and exit
  2. **Interactive mode**: REPL for multiple commands
- Error handling and user feedback

## Data Flow

```
User Input → Parser → Command Struct → JSON Payload → HTTP Client → Telemetry Hub
                ↓
            Validation
```

## Command Grammar

Commands follow this general pattern:
```
command_name(param1=value1, param2=value2, ...)
```

### Parameter Types

- **node_id**: Optional `u32` - if omitted, targets all nodes
- **start_time/end_time**: ISO 8601 timestamp (e.g., `2025-10-23T15:30+01`)
- **active_period/inactive_period**: `u64` seconds
- **log_level**: Enum of `TRACE|DEBUG|INFO|WARN|ERROR`
- **log_filter**: String (substring match)
- **command**: String (raw USB command)

### Timestamp Handling

The parser accepts ISO 8601 timestamps with timezone information and converts them to UTC:

```rust
// Supported formats:
"2025-10-23T15:30:00+01:00"  // RFC 3339
"2025-10-23T15:30+01"         // Short format
"2025-10-23T15:30:00Z"        // UTC
```

All timestamps are converted to UTC and formatted as RFC 3339 before sending to the hub.

## JSON API Format

Commands are sent as JSON to the hub:

```json
{
  "command": "command_name",
  "parameters": {
    "node id": 21,           // Optional
    "param1": "value1",
    ...
  }
}
```

Note: The hub uses `"node id"` (with space) in the JSON, not `node_id`.

## Error Handling

### Parse Errors

- Missing required parameters
- Invalid parameter types
- Unknown command names
- Malformed timestamps

All parse errors are reported to the user without sending a request.

### HTTP Errors

- **401 Unauthorized**: Terminates in single-command mode, exits in interactive mode
- **400 Bad Request**: Reports error, continues
- **5xx Server Error**: Reports error, continues (user can retry)
- **Network errors**: Reports error with context

## Testing

Run the test suite:

```bash
cargo test
```

### Test Coverage

- Command parsing for all command types
- Node ID handling (with and without)
- Quit command detection
- Parameter extraction

### Adding Tests

Tests are located in each module using `#[cfg(test)]` blocks:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        // Test code
    }
}
```

## Building and Running

### Development Build

```bash
cargo build
cargo run -- --config config.toml
```

### Release Build

```bash
cargo build --release
./target/release/moonblokz-telemetry-cli
```

### With Verbose Logging

```bash
RUST_LOG=debug cargo run
```

## Dependencies

Key dependencies and their purposes:

- `tokio` - Async runtime
- `reqwest` - HTTP client with TLS support
- `serde` + `serde_json` - JSON serialization
- `toml` - Configuration file parsing
- `clap` - Command-line argument parsing
- `chrono` - Timestamp parsing and conversion
- `anyhow` + `thiserror` - Error handling

## Extending the CLI

### Adding a New Command

1. **Add command variant to `Command` enum** in `parser.rs`:

```rust
pub enum Command {
    // ... existing variants
    MyNewCommand {
        node_id: Option<u32>,
        my_param: String,
    },
}
```

2. **Add JSON conversion** in `Command::to_json()`:

```rust
Command::MyNewCommand { node_id, my_param } => {
    let mut params = json!({ "my_param": my_param });
    if let Some(id) = node_id {
        params["node id"] = json!(id);
    }
    Ok(json!({
        "command": "my_new_command",
        "parameters": params,
    }))
}
```

3. **Add parser function**:

```rust
fn parse_my_new_command(params_str: &str) -> Result<Command> {
    let params = parse_params(params_str)?;
    let node_id = parse_node_id(&params)?;
    let my_param = get_param(&params, "my_param")
        .ok_or_else(|| anyhow!("Missing my_param"))?
        .to_string();
    
    Ok(Command::MyNewCommand { node_id, my_param })
}
```

4. **Add to command dispatcher** in `parse_command()`:

```rust
match cmd_lower.as_str() {
    // ... existing matches
    "my_new_command" => {
        let params = params_str.ok_or_else(|| anyhow!("my_new_command requires parameters"))?;
        parse_my_new_command(params)
    }
    // ...
}
```

5. **Add tests**:

```rust
#[test]
fn test_parse_my_new_command() {
    let cmd = parse_command("my_new_command(node_id=21, my_param=value)").unwrap();
    match cmd {
        Command::MyNewCommand { node_id, my_param } => {
            assert_eq!(node_id, Some(21));
            assert_eq!(my_param, "value");
        }
        _ => panic!("Wrong command type"),
    }
}
```

## Code Style

The project follows Rust idioms:

- Use `Result<T>` and `?` operator for error propagation
- Avoid unnecessary cloning (pass by reference where possible)
- Use `anyhow::Result` for application errors
- Use `thiserror` for library-style errors (if needed)
- Prefer explicit error messages with context
- Keep functions focused and testable

## Performance Considerations

- HTTP client is reused across requests in interactive mode
- Minimal allocations in the hot path
- Async I/O for network operations
- Efficient string parsing without regex overhead

## Security

- TLS verification is enabled by default (via `reqwest`)
- API keys are read from config file (never hardcoded)
- No sensitive data is logged
- Config file should have restrictive permissions (e.g., `chmod 600 config.toml`)

## Troubleshooting

### Common Issues

**"Failed to load configuration"**
- Ensure `config.toml` exists in the current directory or specify with `--config`
- Check TOML syntax

**"401 Unauthorized"**
- Verify API key in config.toml matches the hub's `cli_api_key`

**"Failed to send request to hub"**
- Check network connectivity
- Verify hub URL is correct and reachable
- Check firewall settings

**"Invalid ISO 8601 timestamp"**
- Ensure timestamps include timezone information
- Use format: `YYYY-MM-DDTHH:MM:SS+HH:MM` or `YYYY-MM-DDTHH:MM+HH:MM`

## Future Enhancements

Potential improvements:

- Command history in interactive mode (using `rustyline`)
- Tab completion for commands
- Configuration validation on startup
- Batch command file support
- Better error messages with suggestions
- Command output formatting options (JSON, table, etc.)
- Dry-run mode to preview JSON payloads
