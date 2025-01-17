# stop-nagging

stop-nagging is a Rust-based CLI tool that silences or disables upgrade/advertising nags and other unnecessary warnings from various JavaScript ecosystem tools (pnpm, npm, vercel, prisma, node, etc.).  
It uses a YAML file (`tools.yaml`) to list each tool's name, environment variables, and commands to run, making it easy for new contributors to update the logic without writing Rust code.

## Features

- Uses a centralized `tools.yaml` to define each tool's disabling strategy (commands, env vars, etc.)
- Configurable via CLI flags (planned)
- Installs easily with `curl`

## Supported Tools

Head over to [`tools.yaml`](tools.yaml) to see the list of supported tools.

## Installation

### Using the Shell Script

### Quick Install

```bash
curl -s https://raw.githubusercontent.com/youruser/stop-nagging/main/scripts/install_stop_nagging.sh | bash
```

Then add `~/.local/bin` to your PATH if not already.

### From Source

1. Ensure Rust is installed
2. Clone the repository:
   ```bash
   git clone https://github.com/youruser/stop-nagging
   ```
3. Build and install:
   ```bash
   cd stop-nagging
   cargo install --path .
   ```
4. Run:
   ```bash
   stop-nagging
   ```

## Usage

```bash
stop-nagging [options]
```

- Without arguments: Runs with default settings, reading `tools.yaml` in the project directory (or local directory)
- Example:
  ```bash
  stop-nagging
  ```

## Contributing

1. Fork the repo and create a new branch
2. Edit `tools.yaml` if you want to add or change how a tool's nagging is disabled
3. If more complex logic is needed, you can add or modify Rust code in `src/runner.rs`
4. Submit a Pull Request

```

```
