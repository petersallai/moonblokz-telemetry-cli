# Developer Documentation

## Architecture

The MoonBlokz Telemetry CLI is built in Rust using Tokio for async runtime and follows a modular architecture.

### Module Structure

```text
src/
├── main.rs       - Entry point, CLI argument parsing, REPL implementation
├── config.rs     - Configuration loading from TOML
├── parser.rs     - Command grammar parser and JSON conversion
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

Implements the current command grammar parser with the following features:

- **Case-insensitive** command parsing
- **Case-insensitive** parameter-key lookup
- **Parameter parsing** with support for:
  - integer node IDs
  - ISO 8601 timestamps with timezone conversion to UTC
  - unquoted string values
  - double-quoted values for comma grouping
  - enumerated values for log levels
- **Current command variants**:
  - `SetUpdateInterval` - Global scheduling parameters
  - `SetLogLevel` - Verbosity control
  - `SetLogFilter` - Filter string updates
  - `Command` - Raw USB command transport via JSON command name `run_command`
  - `UpdateNode` - Firmware updates for RP2040
  - `UpdateProbe` - Probe self-updates
  - `RebootProbe` - Raspberry Pi reboot
  - `StartMeasurement` - Start measurement sequence (`node_id` required)
  - `Quit` - Exit interactive mode

Each command converts to JSON format matching the current hub request contract.

#### Important current parser limitations

- The top-level parser currently accepts `run_command(...)`, not `command(...)`.
- `set_update_interval(...)` currently rejects `node_id` and therefore behaves as a global CLI command.
- Double quotes are preserved in parsed string values rather than stripped after tokenization.

#### 3. Client Module (`client.rs`)

- Uses `reqwest` for HTTP/HTTPS communication
- Sends POST requests to the `/command` endpoint
- Handles HTTP status codes:
  - `200 OK` → Success
  - `401 Unauthorized` → Invalid API key
  - other `4xx` → Client errors
  - `5xx` → Server errors
- Uses a 30-second timeout for requests

#### 4. Main Module (`main.rs`)

- CLI argument parsing with `clap`
- Two modes of operation:
  1. **Single command mode**: Execute one command and exit
  2. **Interactive mode**: REPL for multiple commands
- Error handling and user feedback
- Exit-on-auth-failure behavior in interactive mode

## Data Flow

```text
User Input → Parser → Command Enum → JSON Payload → HTTP Client → Telemetry Hub
                ↓
            Local validation
```

## Command Grammar

Commands follow this general pattern:

```text
command_name(param1=value1, param2=value2, ...)
```

### Current accepted command names

- `set_update_interval`
- `set_log_level`
- `set_log_filter`
- `run_command`
- `update_node`
- `update_probe`
- `reboot_probe`
- `start_measurement`
- `quit`
- `exit`
- `bye`

### Parameter Types

- **node_id**: Optional `u32` for most commands, required for `start_measurement`
- **start_time/end_time**: ISO 8601 timestamp with timezone
- **active_period/inactive_period**: `u64` seconds
- **log_level**: Enum of `TRACE|DEBUG|INFO|WARN|ERROR`
- **log_filter**: String value
- **command**: String value for `run_command(...)`
- **sequence**: `u32` measurement sequence number

### Timestamp Handling

The parser accepts ISO 8601 timestamps with timezone information and converts them to UTC.

Supported examples include:

```text
2025-10-23T15:30:00+01:00
2025-10-23T15:30+01
2025-10-23T15:30:00Z
```

All timestamps are converted to UTC and formatted as RFC 3339 before sending to the hub.

## JSON API Format

Commands are sent as JSON to the hub in the form:

```json
{
  "command": "command_name",
  "parameters": {
    "node id": 21,
    "param1": "value1"
  }
}
```

Important details:

- The CLI input syntax uses `node_id`
- The JSON payload uses `"node id"` with a space
- The raw-command command name sent to the hub is `run_command`

## Error Handling

### Parse Errors

Examples of locally detected parse errors:

- missing required parameters
- invalid parameter types
- unknown command names
- malformed timestamps
- missing closing parenthesis
- `node_id` supplied for `set_update_interval`

All parse errors are reported to the user without sending a request.

### HTTP Errors

- **401 Unauthorized**:
  - single-command mode exits with status 1
  - interactive mode prints an authentication hint and exits with status 1
- **Other 4xx**: reports error and continues in interactive mode
- **5xx Server Error**: reports error and continues in interactive mode
- **Network errors**: reports error with context

## Testing

Run the test suite:

```bash
cargo test
```

### Current test coverage

The repository currently includes parser tests for:

- quit command detection
- set log level parsing
- update node parsing with and without `node_id`
- start measurement parsing
- start measurement requiring `node_id`

### Recommended future coverage

Add tests for areas that are currently especially drift-prone:

- `run_command(...)` acceptance
- rejection of `command(...)`
- rejection of `set_update_interval(node_id=...)`
- quoted string behavior
- invalid timestamp formats

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

### Single Command Example

```bash
cargo run -- --command "run_command(node_id=21, command=/LT)"
```

## Dependencies

Key dependencies and their purposes:

- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde` + `serde_json` - JSON serialization
- `toml` - Configuration file parsing
- `clap` - Command-line argument parsing
- `chrono` - Timestamp parsing and conversion
- `anyhow` + `thiserror` - Error handling

## Extending the CLI

### Adding a New Command

1. **Add command variant to `Command` enum** in `parser.rs`
2. **Add JSON conversion** in `Command::to_json()`
3. **Add parser function** for the new command
4. **Add the command name to the dispatcher** in `parse_command()`
5. **Add tests** for both valid and invalid forms
6. **Update docs and examples** so the public command syntax matches the actual parser

### Important extension rules from current implementation

- If a command is meant to be accepted from the CLI, its exact top-level name must be present in `parse_command()`.
- If a string parameter is intended to support quoted input cleanly, parser behavior may need to be improved because current logic preserves quote characters.
- If a command is meant to be node-scoped, ensure the parser, JSON conversion, hub handling, and docs all agree on that scope.

## Performance Considerations

- The HTTP client is reused across requests in interactive mode
- The parser is simple string processing without regex overhead
- The tool is network-bound in normal usage

## Security

- TLS verification is enabled by default via `reqwest`
- API keys are read from config file and not hardcoded
- No local secret store is used
- Config file should have restrictive permissions such as `chmod 600 config.toml`

## Troubleshooting

### Common Issues

**"Failed to load configuration"**
- Ensure `config.toml` exists in the current directory or specify it with `--config`
- Check TOML syntax

**"401 Unauthorized"**
- Verify the API key in `config.toml` matches the hub's `cli_api_key`

**"Failed to send request to hub"**
- Check network connectivity
- Verify the hub URL is correct and reachable

**"Unknown command: command"**
- The current parser accepts `run_command(...)`, not `command(...)`

**"set_update_interval does not accept node_id parameter"**
- The current parser treats `set_update_interval(...)` as global-only

## Future Enhancements

Potential improvements:

- command history in interactive mode
- tab completion for commands
- configuration validation on startup
- better parser support for quoted strings
- a compatibility alias from `command(...)` to `run_command(...)`
- output formatting options
- dry-run mode to preview JSON payloads
