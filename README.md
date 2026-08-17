# MoonBlokz Telemetry CLI

A command-line interface for sending commands to MoonBlokz probes via the telemetry hub.

## Features

- **Interactive Mode**: Enter commands interactively with a REPL interface
- **Single Command Mode**: Execute a single command and exit
- **Command Types Supported**:
  - `set_update_interval` - Modify the global upload schedule used by the hub
  - `set_log_level` - Change node verbosity levels
  - `set_log_filter` - Update log filtering
  - `run_command` - Send arbitrary USB commands to nodes
  - `update_node` - Trigger node firmware updates
  - `update_probe` - Trigger probe self-updates
  - `reboot_probe` - Reboot probe Raspberry Pi
  - `start_measurement` - Start a measurement sequence on a node

## Configuration

Create a `config.toml` file with the following settings:

```toml
# API key to authenticate with the hub's /command endpoint
api-key = "your-cli-api-key-here"

# Base URL of the hub (without the /command suffix)
hub-url = "https://your-hub-url.example.com"
```

## Installation

Build the application:

```bash
cargo build --release
```

The binary will be located at `target/release/moonblokz-telemetry-cli`.

## Usage

### Interactive Mode

Run without arguments to enter interactive mode:

```bash
moonblokz-telemetry-cli
```

Or specify a custom config file:

```bash
moonblokz-telemetry-cli --config /path/to/config.toml
```

### Single Command Mode

Execute a single command and exit:

```bash
moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=DEBUG)"
```

## Command Syntax

Commands follow this general form:

```text
command_name(param1=value1, param2=value2, ...)
```

Notes about the current parser:

- command names are case-insensitive
- input parameter names are matched case-insensitively
- `node_id` is optional for most commands
- `node_id` is required for `start_measurement`
- `set_update_interval` currently does **not** accept `node_id`
- the current parser accepts `run_command(...)`, not `command(...)`

## Supported Commands

### Set Update Interval

Change the upload schedule for all probes through the hub’s global interval policy:

```text
set_update_interval(start_time=2025-10-23T15:30+01, end_time=2025-10-23T18:00+01, active_period=60, inactive_period=300)
```

`start_time` and `end_time` must be valid ISO 8601 timestamps with timezone information.

### Set Log Level

Change the verbosity of a node:

```text
set_log_level(node_id=21, log_level=DEBUG)
```

Or target all nodes currently known to the hub:

```text
set_log_level(log_level=INFO)
```

Valid log levels: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`

### Set Log Filter

Update the substring filter:

```text
set_log_filter(node_id=21, log_filter=[ERROR])
```

Or target all nodes:

```text
set_log_filter(log_filter=[WARN])
```

### Send Arbitrary Command

Send a raw USB command to a node:

```text
run_command(node_id=21, command=/LT)
```

Or target all nodes:

```text
run_command(command=/LI)
```

### Update Node Firmware

Trigger a node firmware update:

```text
update_node(node_id=21)
```

Or target all nodes:

```text
update_node()
```

### Update Probe Firmware

Trigger a probe self-update:

```text
update_probe(node_id=21)
```

Or target all nodes:

```text
update_probe()
```

### Reboot Probe

Reboot the Raspberry Pi:

```text
reboot_probe(node_id=21)
```

Or target all nodes:

```text
reboot_probe()
```

### Start Measurement

Start a measurement sequence on a specific node (`node_id` is required):

```text
start_measurement(node_id=21, sequence=1)
```

## Important Current Limitations

- `OK` means the hub accepted the command request. It does **not** mean a probe already executed it.
- `set_update_interval(...)` is currently global-only from the CLI side.
- The parser currently preserves double quotes inside parsed string values. To avoid ambiguity, examples in this file use unquoted string values where practical.

## Exit Commands

In interactive mode, use any of these to exit:

- `quit`
- `exit`
- `bye`

## Error Handling

- **401 Unauthorized**: Invalid API key - check your configuration
- **4xx Client Error**: Invalid command payload or hub-side validation failure
- **5xx Server Error**: Hub server error - retry later
- **Parse Errors**: Check command syntax and parameter types

In interactive mode, a 401 error also causes the CLI to exit after printing an authentication hint.

## Examples

```bash
# Enter interactive mode
$ moonblokz-telemetry-cli
MoonBlokz Telemetry CLI - Interactive Mode
Type 'quit', 'exit', or 'bye' to exit

> set_log_level(node_id=21, log_level=DEBUG)
OK
> update_node(node_id=21)
OK
> start_measurement(node_id=21, sequence=1)
OK
> quit
Goodbye!

# Single command
$ moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=INFO)"
OK
$ moonblokz-telemetry-cli --command "run_command(node_id=21, command=/LT)"
OK
```

## Development

Run tests:

```bash
cargo test
```

## License

See LICENSE file for details.
