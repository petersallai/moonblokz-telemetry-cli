# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust CLI for operating MoonBlokz field-test nodes through the Telemetry HUB.

- `src/main.rs`: CLI entrypoint, argument parsing, interactive REPL, single-command execution.
- `src/parser.rs`: command grammar, validation, and JSON payload conversion.
- `src/client.rs`: async HTTP client (`/command` calls, status/error handling).
- `src/config.rs`: loads `config.toml` (`api-key`, `hub-url`).
- Root docs: `README.md`, `DEVELOPER.md`, `EXAMPLES.md`, `CHANGELOG.md`.

## Build, Test, and Development Commands
Use Cargo for all local workflows:

- `cargo build`: compile debug build.
- `cargo build --release`: produce optimized binary at `target/release/moonblokz-telemetry-cli`.
- `cargo run -- --config config.toml`: run interactively with explicit config path.
- `cargo run -- --command "run_command(node_id=21, command=/LT)"`: execute one command and exit.
- `cargo test`: run unit tests.

## Coding Style & Naming Conventions
Follow idiomatic Rust conventions:

- Use `rustfmt` defaults (4-space indentation, trailing commas where appropriate).
- Prefer `snake_case` for functions/modules/variables and `CamelCase` for types/enums.
- Keep module boundaries clear: parsing in parser, network logic in client, config loading in config.
- Preserve hub payload compatibility (`"node id"` JSON key, command names such as `run_command`).

## Domain Context (From Radio/Field Infrastructure)
- Treat the network as best-effort and lossy; avoid CLI behavior that assumes guaranteed packet delivery.
- This CLI transmits commands to the Telemetry HUB over HTTP (`/command`); it does not transmit directly over LoRa.
- `start_measurement(node_id=..., sequence=...)` is a field-test command for measuring `add_block` propagation; keep `node_id` mandatory.
- The Telemetry HUB is the control/data aggregation layer between distributed probes and this CLI; changes should remain hub-centric, not device-direct.

## Testing Guidelines
- Add unit tests next to implementation using `#[cfg(test)] mod tests`.
- Use descriptive `test_*` names (for example, `test_parse_start_measurement_requires_node_id`).
- Cover both valid and invalid command inputs whenever parser behavior changes.
- Run `cargo test` before opening a PR.

## Commit & Pull Request Guidelines
Recent commits use short, imperative subjects (for example, "Rename ...", "Refactor ...", "Add ...").

- Keep commit titles concise and action-oriented; one logical change per commit.
- In PRs, include: purpose, behavior changes, test evidence (`cargo test` output summary), and any config/doc updates.
- Link related issues/tasks and include example CLI commands when changing command syntax.

## Security & Configuration Tips
- Never commit real API keys or environment-specific hub URLs.
- Keep secrets in local `config.toml` only; use placeholders in docs and examples.
- Avoid adding logs or examples that expose node location/identity data from field deployments.

## Further Reference
- MoonBlokz Series Part VII/5 (Field Testing Infrastructure): https://medium.com/moonblokz/moonblokz-series-part-vii-5-field-testing-infrastructure-6be10e18796c

