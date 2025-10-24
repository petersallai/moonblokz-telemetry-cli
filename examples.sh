#!/bin/bash
# Example usage of the MoonBlokz Telemetry CLI

# Make sure to update config.toml with your actual API key and hub URL first!

echo "MoonBlokz Telemetry CLI - Example Commands"
echo "==========================================="
echo ""

# Build the application first
echo "Building the application..."
cargo build --release
echo ""

# Path to the binary
CLI="./target/release/moonblokz-telemetry-cli"

echo "Example 1: Set log level for node 21 to DEBUG"
$CLI --command "set_log_level(node_id=21, log_level=DEBUG)"
echo ""

echo "Example 2: Set log filter for node 21"
$CLI --command "set_log_filter(node_id=21, log_filter=\"[ERROR]\")"
echo ""

echo "Example 3: Update firmware for node 21"
$CLI --command "update_node(node_id=21)"
echo ""

echo "Example 4: Update all probes"
$CLI --command "update_probe()"
echo ""

echo "Example 5: Set update interval for node 21"
$CLI --command "set_update_interval(node_id=21, start_time=2025-10-23T15:30+01, end_time=2025-10-23T18:00+01, active_period=60, inactive_period=300)"
echo ""

echo "Example 6: Send arbitrary command to node 21"
$CLI --command "command(node_id=21, command=\"/LT\")"
echo ""

echo ""
echo "For interactive mode, run: $CLI"
echo "In interactive mode, you can enter commands one at a time."
echo "Type 'quit', 'exit', or 'bye' to exit interactive mode."
