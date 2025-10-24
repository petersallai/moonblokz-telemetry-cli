# MoonBlokz Telemetry CLI

A command-line interface for sending commands to MoonBlokz probes via the telemetry hub.

## Features

- **Interactive Mode**: Enter commands interactively with a REPL interface
- **Single Command Mode**: Execute a single command and exit
- **Command Types Supported**:
  - `set_update_interval` - Modify probe upload schedules
  - `set_log_level` - Change node verbosity levels
  - `set_log_filter` - Update log filtering
  - `command` - Send arbitrary USB commands to nodes
  - `update_node` - Trigger node firmware updates
  - `update_probe` - Trigger probe self-updates
  - `reboot_probe` - Reboot probe Raspberry Pi

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

### Set Update Interval

Change the upload schedule for a probe:

```
set_update_interval(node_id=21, start_time=2025-10-23T15:30+01, end_time=2025-10-23T18:00+01, active_period=60, inactive_period=300)
```

Omit `node_id` to target all probes:

```
set_update_interval(start_time=2025-10-23T15:30+01, end_time=2025-10-23T18:00+01, active_period=60, inactive_period=300)
```

### Set Log Level

Change the verbosity of a node:

```
set_log_level(node_id=21, log_level=DEBUG)
```

Valid log levels: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`

### Set Log Filter

Update the substring filter:

```
set_log_filter(node_id=21, log_filter="[ERROR]")
```

### Send Arbitrary Command

Send a raw USB command to a node:

```
command(node_id=21, command="/LT")
```

### Update Node Firmware

Trigger a node firmware update:

```
update_node(node_id=21)
```

Or target all nodes:

```
update_node()
```

### Update Probe Firmware

Trigger a probe self-update:

```
update_probe(node_id=21)
```

### Reboot Probe

Reboot the Raspberry Pi:

```
reboot_probe(node_id=21)
```

## Exit Commands

In interactive mode, use any of these to exit:
- `quit`
- `exit`
- `bye`

## Error Handling

- **401 Unauthorized**: Invalid API key - check your configuration
- **400 Bad Request**: Invalid command syntax or parameters
- **5xx Server Error**: Hub server error - retry later
- **Parse Errors**: Check command syntax and parameter types

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
> quit
Goodbye!

# Single command
$ moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=INFO)"
OK
```

## Development

Run tests:

```bash
cargo test
```

Run with debug output:

```bash
RUST_LOG=debug cargo run
```

## License

See LICENSE file for details.
A command line interface to give commands to a moonblokz-telemetry-hu
