# Command Examples and Test Scenarios

This document provides practical examples and test scenarios for the MoonBlokz Telemetry CLI.

## Important Notes About Current Behavior

Before using the examples below, keep these current implementation details in mind:

- the parser accepts `run_command(...)`, not `command(...)`
- `set_update_interval(...)` currently does **not** accept `node_id`
- `start_measurement(...)` requires `node_id`
- the CLI prints `OK` when the hub accepts the request; this does not prove later Probe execution
- to avoid ambiguity with the current parser, examples below prefer unquoted string values where practical

## Configuration Setup

First, ensure your `config.toml` is properly configured:

```toml
api-key = "your-cli-api-key-here"
hub-url = "https://your-hub-url.example.com"
```

## Basic Command Examples

### 1. Set Log Level

Change the log level for a specific node:

```bash
moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=DEBUG)"
```

Change log level for all nodes:

```bash
moonblokz-telemetry-cli --command "set_log_level(log_level=INFO)"
```

Valid log levels: `TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`

### 2. Set Log Filter

Filter logs to show only errors for node 21:

```bash
moonblokz-telemetry-cli --command "set_log_filter(node_id=21, log_filter=[ERROR])"
```

Clear filter for node 21:

```bash
moonblokz-telemetry-cli --command "set_log_filter(node_id=21, log_filter=)"
```

Apply a filter to all nodes:

```bash
moonblokz-telemetry-cli --command "set_log_filter(log_filter=[WARN])"
```

### 3. Update Interval

Set the global upload interval policy used by the hub:

```bash
moonblokz-telemetry-cli --command "set_update_interval(start_time=2025-10-23T09:00:00+00:00, end_time=2025-10-23T17:00:00+00:00, active_period=60, inactive_period=300)"
```

This command tells the hub to use:

- 60-second intervals during the active window
- 300-second intervals outside the active window

### 4. Send USB Commands

Send a raw USB command to a node:

```bash
moonblokz-telemetry-cli --command "run_command(node_id=21, command=/LT)"
```

Send a raw USB command to all nodes known to the hub:

```bash
moonblokz-telemetry-cli --command "run_command(command=/LI)"
```

Common USB commands:

- `/LT` - Set log level to TRACE
- `/LD` - Set log level to DEBUG
- `/LI` - Set log level to INFO
- `/LW` - Set log level to WARN
- `/LE` - Set log level to ERROR

### 5. Firmware Updates

Update node firmware for node 21:

```bash
moonblokz-telemetry-cli --command "update_node(node_id=21)"
```

Update node firmware for all nodes:

```bash
moonblokz-telemetry-cli --command "update_node()"
```

Update probe firmware for node 21:

```bash
moonblokz-telemetry-cli --command "update_probe(node_id=21)"
```

Update all probes:

```bash
moonblokz-telemetry-cli --command "update_probe()"
```

### 6. Reboot Probe

Reboot the Raspberry Pi for node 21:

```bash
moonblokz-telemetry-cli --command "reboot_probe(node_id=21)"
```

Reboot all probes:

```bash
moonblokz-telemetry-cli --command "reboot_probe()"
```

### 7. Start Measurement

Start a measurement sequence on a specific node:

```bash
moonblokz-telemetry-cli --command "start_measurement(node_id=21, sequence=1)"
```

Start a different measurement sequence:

```bash
moonblokz-telemetry-cli --command "start_measurement(node_id=42, sequence=100)"
```

## Interactive Mode Examples

### Session 1: Basic Monitoring Setup

```text
$ moonblokz-telemetry-cli
MoonBlokz Telemetry CLI - Interactive Mode
Type 'quit', 'exit', or 'bye' to exit

> set_log_level(node_id=21, log_level=DEBUG)
OK
> set_log_filter(node_id=21, log_filter=[DEBUG])
OK
> start_measurement(node_id=21, sequence=1)
OK
> quit
Goodbye!
```

### Session 2: Firmware Update Sequence

```text
$ moonblokz-telemetry-cli
MoonBlokz Telemetry CLI - Interactive Mode
Type 'quit', 'exit', or 'bye' to exit

> set_log_level(node_id=21, log_level=INFO)
OK
> update_node(node_id=21)
OK
> exit
Goodbye!
```

### Session 3: Mass Configuration

```text
$ moonblokz-telemetry-cli
MoonBlokz Telemetry CLI - Interactive Mode
Type 'quit', 'exit', or 'bye' to exit

> set_log_level(log_level=INFO)
OK
> set_update_interval(start_time=2025-10-23T08:00:00-05:00, end_time=2025-10-23T18:00:00-05:00, active_period=60, inactive_period=600)
OK
> update_probe()
OK
> bye
Goodbye!
```

## Timezone Examples

The CLI accepts timestamps with various timezone formats and converts them to UTC before submission.

### UTC Time

```bash
moonblokz-telemetry-cli --command "set_update_interval(start_time=2025-10-23T14:00:00Z, end_time=2025-10-23T22:00:00Z, active_period=60, inactive_period=300)"
```

### US Eastern Time (UTC-5)

```bash
moonblokz-telemetry-cli --command "set_update_interval(start_time=2025-10-23T09:00:00-05:00, end_time=2025-10-23T17:00:00-05:00, active_period=60, inactive_period=300)"
```

### European Central Time (UTC+1)

```bash
moonblokz-telemetry-cli --command "set_update_interval(start_time=2025-10-23T15:00:00+01:00, end_time=2025-10-23T23:00:00+01:00, active_period=60, inactive_period=300)"
```

## Error Handling Examples

### Parse Errors

Invalid log level:

```bash
$ moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=INVALID)"
Parse error: Invalid log_level: must be TRACE, DEBUG, INFO, WARN, or ERROR
```

Missing parameter:

```bash
$ moonblokz-telemetry-cli --command "set_log_level(node_id=21)"
Parse error: Missing log_level parameter
```

Invalid timestamp:

```bash
$ moonblokz-telemetry-cli --command "set_update_interval(start_time=invalid, end_time=2025-10-23T18:00:00Z, active_period=60, inactive_period=300)"
Parse error: Invalid ISO 8601 timestamp: invalid
```

Invalid node-scoped interval attempt:

```bash
$ moonblokz-telemetry-cli --command "set_update_interval(node_id=21, start_time=2025-10-23T09:00:00Z, end_time=2025-10-23T18:00:00Z, active_period=60, inactive_period=300)"
Parse error: set_update_interval does not accept node_id parameter
```

Missing required node_id for start_measurement:

```bash
$ moonblokz-telemetry-cli --command "start_measurement(sequence=1)"
Parse error: node_id is required for start_measurement command
```

Unknown top-level command alias:

```bash
$ moonblokz-telemetry-cli --command "command(node_id=21, command=/LT)"
Parse error: Unknown command: command
```

### HTTP Errors

Authentication failure:

```bash
$ moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=DEBUG)"
Command error: 401 Unauthorized - Invalid API key
```

Server error:

```bash
$ moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=DEBUG)"
Server error: 500 - Internal server error
```

## Testing Scenarios

### Scenario 1: Development Mode Setup

Goal: Configure all nodes for verbose logging during development hours

```bash
# Set all nodes to DEBUG level
moonblokz-telemetry-cli --command "set_log_level(log_level=DEBUG)"

# Configure active development hours (9 AM - 5 PM local time)
moonblokz-telemetry-cli --command "set_update_interval(start_time=2025-10-23T09:00:00-05:00, end_time=2025-10-23T17:00:00-05:00, active_period=30, inactive_period=600)"
```

### Scenario 2: Production Monitoring

Goal: Set up production-level logging with less frequent updates

```bash
# Set all nodes to INFO level
moonblokz-telemetry-cli --command "set_log_level(log_level=INFO)"

# Filter to show only warnings and errors
moonblokz-telemetry-cli --command "set_log_filter(log_filter=[WARN])"

# Longer update intervals to reduce bandwidth
moonblokz-telemetry-cli --command "set_update_interval(start_time=2025-10-23T00:00:00Z, end_time=2025-10-24T00:00:00Z, active_period=600, inactive_period=600)"
```

### Scenario 3: Emergency Debugging

Goal: Quickly enable detailed logging on a specific node

```bash
# Enable TRACE level on node 42
moonblokz-telemetry-cli --command "set_log_level(node_id=42, log_level=TRACE)"

# Remove any filters
moonblokz-telemetry-cli --command "set_log_filter(node_id=42, log_filter=)"

# Increase global upload frequency to 30 seconds
moonblokz-telemetry-cli --command "set_update_interval(start_time=2025-10-23T00:00:00Z, end_time=2025-10-24T00:00:00Z, active_period=30, inactive_period=30)"
```

### Scenario 4: Firmware Rollout

Goal: Safely update firmware for all nodes

```bash
# Start with one node as a test
moonblokz-telemetry-cli --command "update_node(node_id=21)"

# Wait and verify the update...

# If successful, update all nodes
moonblokz-telemetry-cli --command "update_node()"
```

### Scenario 5: Probe Maintenance

Goal: Update probe software on specific nodes

```bash
# Update probe on node 21
moonblokz-telemetry-cli --command "update_probe(node_id=21)"

# Reboot the probe after update
moonblokz-telemetry-cli --command "reboot_probe(node_id=21)"
```

### Scenario 6: Measurement Sequence

Goal: Start coordinated measurements across multiple nodes

```bash
# Enable detailed logging for nodes involved in measurement
moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=DEBUG)"
moonblokz-telemetry-cli --command "set_log_level(node_id=42, log_level=DEBUG)"

# Start measurement sequence on node 21
moonblokz-telemetry-cli --command "start_measurement(node_id=21, sequence=1)"

# Start measurement sequence on node 42
moonblokz-telemetry-cli --command "start_measurement(node_id=42, sequence=2)"

# Increase global upload frequency to capture measurement data
moonblokz-telemetry-cli --command "set_update_interval(start_time=2025-12-11T00:00:00Z, end_time=2025-12-12T00:00:00Z, active_period=10, inactive_period=10)"
```

## Batch Operations Script

For automating multiple commands, create a shell script:

```bash
#!/bin/bash
# setup-development.sh

CLI="moonblokz-telemetry-cli"

echo "Setting up development environment..."

# Configure logging
$CLI --command "set_log_level(log_level=DEBUG)"
$CLI --command "set_log_filter(log_filter=)"

# Set active upload during work hours (8 AM - 6 PM EST)
$CLI --command "set_update_interval(start_time=2025-12-11T08:00:00-05:00, end_time=2025-12-11T18:00:00-05:00, active_period=60, inactive_period=300)"

echo "Development environment configured!"
```

### Measurement Test Script

```bash
#!/bin/bash
# run-measurement-test.sh

CLI="moonblokz-telemetry-cli"
NODE_ID=21
SEQUENCE=$((RANDOM % 1000 + 1))

echo "Starting measurement test on node $NODE_ID with sequence $SEQUENCE..."

# Configure node for measurement
$CLI --command "set_log_level(node_id=$NODE_ID, log_level=INFO)"
$CLI --command "set_update_interval(start_time=2025-12-11T00:00:00Z, end_time=2025-12-11T01:00:00Z, active_period=15, inactive_period=15)"

# Start measurement
$CLI --command "start_measurement(node_id=$NODE_ID, sequence=$SEQUENCE)"

echo "Measurement sequence $SEQUENCE started on node $NODE_ID"
```

## Troubleshooting Commands

### Check if hub is reachable

```bash
# Try a simple command
moonblokz-telemetry-cli --command "set_log_level(node_id=999, log_level=INFO)"
```

If you get `OK`, the hub is reachable and accepted the request. That does not guarantee that node 999 exists or that a probe will later execute the command.

### Test with different nodes

```bash
# Test node 21
moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=DEBUG)"

# Test node 42
moonblokz-telemetry-cli --command "set_log_level(node_id=42, log_level=DEBUG)"

# Test broadcast (all nodes)
moonblokz-telemetry-cli --command "set_log_level(log_level=INFO)"
```

## Advanced Usage

### Custom Config File Location

```bash
moonblokz-telemetry-cli --config /etc/moonblokz/config.toml --command "set_log_level(node_id=21, log_level=INFO)"
```

### Piping Commands

```bash
echo "set_log_level(node_id=21, log_level=DEBUG)" | moonblokz-telemetry-cli
```

This enters interactive mode and reads the piped input through stdin.

## Integration with Other Tools

### Log command output

```bash
moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=DEBUG)" | tee -a command-log.txt
```

### Error checking in scripts

```bash
#!/bin/bash
if moonblokz-telemetry-cli --command "set_log_level(node_id=21, log_level=DEBUG)"; then
    echo "Command request accepted"
else
    echo "Command request failed" >&2
    exit 1
fi
```

## Performance Notes

- Single command mode is dominated by network round-trip time
- Interactive mode reuses the same HTTP client across commands
- The tool is best suited for direct operator use and simple automation scripts

## Security Best Practices

1. **Protect config file**:

   ```bash
   chmod 600 config.toml
   ```

2. **Avoid committing real API keys**:

   Keep only placeholders in tracked files.

3. **Audit command submission** by logging CLI outputs when needed:

   ```bash
   moonblokz-telemetry-cli | tee -a command-audit.log
   ```

## Summary

This CLI provides interactive and one-shot command submission to the Telemetry HUB. Use single-command mode for automation and scripts, and interactive mode for manual operations and testing.
