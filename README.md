# stop-nagging

`stop-nagging` is a Rust-based CLI tool that silences or disables upgrade/advertising nags and other unnecessary warnings from various CLI tools and development tools. It also disables telemetry and other tracking mechanisms.

It uses a YAML file (`tools.yaml`) to list each tool's name, environment variables, and commands to run, making it easy for new contributors to update the logic without writing Rust code.

> [!WARNING]  
> This tool is very new and may not work for all your tools. If you find a tool that doesn't work, please open an issue or submit a PR to fix it.

## Philosophy

`stop-nagging` is designed to be a fast, simple, and effective tool for disabling nags and warnings. Running `stop-nagging` should be a no-op and it should not modify the source code. Some tools might require configuration changes to stop nagging, we will not modify the source code to do this.

## Supported Tools

Head over to [`tools.yaml`](tools.yaml) to see the list of supported tools.

## Installation

### Quick Install (Linux/macOS)

<!-- LINUX_INSTALLATION_BEGIN -->

```bash
curl -fsSL https://bodo.run/stop-nagging.sh | bash
```

<!-- LINUX_INSTALLATION_END -->

### Quick Install (Windows)

1. Download and run the PowerShell installer script:
   <!-- WINDOWS_INSTALLATION_BEGIN -->
   ```powershell
   iwr https://bodo.run/stop-nagging.ps1 -UseBasicParsing | iex
   ```
   <!-- WINDOWS_INSTALLATION_END -->
2. If needed, add the installation directory (default: `$HOME\.local\bin`) to your PATH.

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

### Options

- `-y, --yaml <FILE>`: Optional path to a custom YAML configuration file
  - If not provided, the default built-in configuration will be used
  - If the custom file fails to load, falls back to the default configuration
  - See [`tools.yaml`](tools.yaml) for the default configuration
- `--ignore-tools <TOOLS>`: Comma-separated list of tool names to ignore (e.g., `npm,yarn`)
- `--ecosystems <ECOSYSTEMS>`: Comma-separated list of ecosystems to run (leave empty to run all)
- `--ignore-ecosystems <ECOSYSTEMS>`: Comma-separated list of ecosystems to skip entirely
- `-v, --verbose`: Enable verbose logging for debugging and detailed progress information

### Examples

```bash
# Run with default built-in configuration
stop-nagging

# Use a custom YAML file
stop-nagging --yaml custom-tools.yaml

# Ignore specific tools (using default configuration)
stop-nagging --ignore-tools npm,yarn,pnpm

# Only run for specific ecosystems (using default configuration)
stop-nagging --ecosystems nodejs,python

# Skip entire ecosystems
stop-nagging --ignore-ecosystems python,rust

# Enable verbose output
stop-nagging --verbose

# Combine multiple options with custom configuration
stop-nagging --yaml custom.yaml --ignore-tools npm --ecosystems nodejs --verbose
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
