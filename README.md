# stop-nagging

stop-nagging is a Rust-based CLI tool that silences or disables upgrade/advertising nags and other unnecessary warnings from various CLI tools and development tools.  
It uses a YAML file (`tools.yaml`) to list each tool's name, environment variables, and commands to run, making it easy for new contributors to update the logic without writing Rust code.

## Features

- Uses a centralized `tools.yaml` to define each tool's disabling strategy (commands, env vars, etc.)
- Configurable via CLI flags (planned)
- Installs easily with `curl`

## Supported Tools

Head over to [`tools.yaml`](tools.yaml) to see the list of supported tools.

## Installation

### Quick Install

```bash
curl -s https://raw.githubusercontent.com/bodo-run/stop-nagging/main/scripts/install_stop_nagging.sh | bash
```

Then add `~/.local/bin` to your PATH if not already.

### From Source

1. Ensure Rust is installed
2. Clone the repository:
   ```bash
   git clone https://github.com/bodo-run/stop-nagging
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

> [!NOTE]  
> `stop-nagging` never exits with an error code. The last thing you want is to have your CI fail because of a tool that's not essential. It will print warnings if any of the steps fails.

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

## Behavior: Non-Failing

`stop-nagging` **never** exits with a nonzero code, even if it fails to disable certain nags. This ensures your CI/CD pipeline won't break due to a missing or optional tool. Instead, it prints **warnings** for:

1. Missing executables (not found in `PATH`).
2. Commands that fail.
3. Already-set environment variables (which we won't override).

You'll see these warnings in the console logs, but your process will exit **0** regardless.

## Skipping Already-Set Environment Variables

If an environment variable is **already set**, `stop-nagging` **does not override** it. This avoids unintentional conflicts with variables you may want set differently. If a var is already set, we print a warning like:

Warning: Env var 'KEY' is already set; skipping override for tool 'npm'.

```

```
