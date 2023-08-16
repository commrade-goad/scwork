use serde_derive::{Deserialize, Serialize};
use std::env;
use std::process::{exit, Command, Output};

#[derive(Deserialize, Serialize)]
struct WorkspaceData {
    num: usize,
    focused: bool,
    output: String,
}

enum ProgMode {
    Prev,
    Next,
}

const DEFAULT_MIN_VAL: usize = 1;
const DEFAULT_MAX_VAL: usize = 10;
const SWAY_COMMAND: &str = "swaymsg";
const SWAY_GET_WORKSPACES: [&str; 2] = ["-t", "get_workspaces"];

fn spawn_process(program: &str, args: Vec<&str>, capture_output: bool) -> Option<String> {
    match capture_output {
        true => {
            let command_output: Output = Command::new(program)
                .args(args)
                .output()
                .expect("ERR: Failed to execute command.");

            if command_output.status.success() {
                let output_str: String =
                    String::from_utf8_lossy(&command_output.stdout).to_string();
                return Some(output_str);
            }

            let error_str = String::from_utf8_lossy(&command_output.stderr);
            eprintln!("ERR: Command execution failed -> {}", error_str);
            return None;
        }
        false => {
            // if no capture output mode will return empty string
            Command::new(program)
                .args(args)
                .spawn()
                .expect("ERR: Failed to execute command.")
                .wait()
                .expect("ERR: Failed to wait the command");
            return Some("".to_string());
        }
    }
}

fn parse_json(json_str: &str) -> Option<Vec<WorkspaceData>> {
    match serde_json::from_str(json_str) {
        Ok(val) => return Some(val),
        Err(_) => return None,
    };
}

fn get_args() -> Vec<String> {
    let tmp_vec: Vec<String> = env::args().collect();
    return tmp_vec;
}

fn calculate_workspaces(
    mode: ProgMode,
    workspaces: Vec<WorkspaceData>,
    min: usize,
    max: usize,
) -> Option<usize> {
    for workspace in workspaces {
        if workspace.focused == true {
            let mut num: usize = workspace.num;
            match mode {
                ProgMode::Prev => {
                    if num <= min {
                        return Some(max);
                    }
                    num -= 1;
                    return Some(num);
                }
                ProgMode::Next => {
                    if num >= max {
                        return Some(min);
                    }
                    num += 1;
                    return Some(num);
                }
            }
        }
    }
    return None;
}

fn main() {
    let user_args: Vec<String> = get_args();
    let mut min: usize = DEFAULT_MIN_VAL;
    let mut max: usize = DEFAULT_MAX_VAL;
    if user_args.len() < 2 {
        eprintln!("ERR: Not enought args.");
        exit(1);
    }
    for idx in 0..user_args.len() {
        match &user_args[idx][..] {
            "-min" => {
                if idx + 2 > user_args.len() {
                    min = DEFAULT_MIN_VAL;
                } else {
                    min = user_args[idx + 1].trim().parse().unwrap_or(DEFAULT_MIN_VAL);
                }
            }
            "-max" => {
                if idx + 2 > user_args.len() {
                    max = DEFAULT_MAX_VAL;
                } else {
                    max = user_args[idx + 1].trim().parse().unwrap_or(DEFAULT_MAX_VAL);
                }
            }
            _ => {}
        }
    }

    let mut args_set_workspaces: Vec<&str> = vec!["workspace"];
    let workspace_num: usize;
    let json_output: String = match spawn_process(SWAY_COMMAND, SWAY_GET_WORKSPACES.to_vec(), true) {
        Some(val) => val,
        None => {
            eprintln!("ERR: Failed to spawn process {}", SWAY_COMMAND);
            exit(1);
        }
    };
    let json_data: Vec<WorkspaceData> = match parse_json(&json_output) {
        Some(val) => val,
        None => {
            eprintln!("ERR: Failed to parse process json output");
            exit(1);
        }
    };
    match &user_args[1][..] {
        "p" | "prev" => {
            workspace_num = match calculate_workspaces(ProgMode::Prev, json_data, min, max) {
                Some(val) => val,
                None => {
                    eprintln!("ERR: Failed to calculate the prev workspace.");
                    exit(1);
                }
            };
        }
        "n" | "next" => {
            workspace_num = match calculate_workspaces(ProgMode::Next, json_data, min, max) {
                Some(val) => val,
                None => {
                    eprintln!("ERR: Failed to calculate the next workspace.");
                    exit(1);
                }
            };
        }
        _ => exit(1),
    }
    let workspace_num_str: &str = &workspace_num.to_string();
    args_set_workspaces.push(workspace_num_str);
    spawn_process(SWAY_COMMAND, args_set_workspaces, false);
}
