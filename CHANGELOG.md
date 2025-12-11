# Changelog

## [Unreleased]

### Added
- New `start_measurement` command to initiate measurement sequences on nodes
  - Syntax: `start_measurement(node_id=<id>, sequence=<number>)`
  - Note: `node_id` parameter is **required** for this command (unlike other commands where it's optional)
  - `sequence` is a `u32` number identifying the measurement sequence
  - Example: `start_measurement(node_id=21, sequence=1)`
  
### Changed
- Updated documentation (README.md, DEVELOPER.md, EXAMPLES.md, PROJECT_SUMMARY.md) to include `start_measurement` command
- Added comprehensive examples for the new command in EXAMPLES.md
- Updated examples.sh script with `start_measurement` usage

### Technical Details

- Added `StartMeasurement` variant to `Command` enum with required `node_id: u32` and `sequence: u32` fields
- Implemented JSON conversion for the new command
- Added `parse_start_measurement` function with validation that `node_id` is present
- Added unit tests to verify parsing and node_id requirement
- All tests pass (6/6)

## [0.1.0] - 2025-12-11

### Initial Release
- Interactive REPL mode and single-command mode
- Support for 7 command types:
  - `set_update_interval` - Modify probe upload schedules
  - `set_log_level` - Change node verbosity
  - `set_log_filter` - Update log filtering
  - `command` - Send arbitrary USB commands
  - `update_node` - Trigger node firmware updates
  - `update_probe` - Trigger probe self-updates
  - `reboot_probe` - Reboot Raspberry Pi
- Configuration via TOML file
- ISO 8601 timestamp parsing with timezone support
- Comprehensive error handling
- Full documentation and examples
