use std::process::Command;

pub fn check_executable(executable: &str) -> Result<(), String> {
    if cfg!(windows) {
        let where_cmd = format!("where {}", executable);
        match Command::new("cmd").args(["/C", &where_cmd]).output() {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err(format!("'{}' not found in PATH", executable)),
        }
    } else {
        let which_cmd = format!("command -v {}", executable);
        match Command::new("sh").arg("-c").arg(&which_cmd).output() {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err(format!("'{}' not found in PATH", executable)),
        }
    }
}

pub fn run_shell_command(cmd: &str) -> Result<(), String> {
    if cfg!(windows) {
        match Command::new("cmd").args(["/C", cmd]).output() {
            Ok(output) if output.status.success() => Ok(()),
            Ok(output) => Err(String::from_utf8_lossy(&output.stderr).to_string()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        match Command::new("sh").arg("-c").arg(cmd).output() {
            Ok(output) if output.status.success() => Ok(()),
            Ok(output) => Err(String::from_utf8_lossy(&output.stderr).to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn local_bin_path(tool: &str) -> String {
    if cfg!(windows) {
        format!(".\\node_modules\\.bin\\{}.cmd", tool)
    } else {
        format!("./node_modules/.bin/{}", tool)
    }
}
