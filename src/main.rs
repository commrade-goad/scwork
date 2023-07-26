use serde_derive::{Deserialize, Serialize};
use std::env;
use std::process;
use std::process::{Command, Output};

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

// TODO: add min & max
fn spawn_process(program: &str, args: Vec<&str>, capture_output: bool) -> Result<String, ()> {
    match capture_output {
        true => {
            let command_output: Output = Command::new(program)
                .args(args)
                .output()
                .expect("ERR: Failed to execute command.");

            if command_output.status.success() {
                let output_str: String =
                    String::from_utf8_lossy(&command_output.stdout).to_string();
                return Ok(output_str);
            }

            let error_str = String::from_utf8_lossy(&command_output.stderr);
            eprintln!("Command execution failed: {}", error_str);
            return Err(());
        }
        false => {
            // if no capture output mode will return empty string
            Command::new(program)
                .args(args)
                .spawn()
                .expect("ERR: Failed to execute command.")
                .wait()
                .expect("ERR: Failed to wait the command");
            return Ok("".to_string());
        }
    }
}

fn parse_json(json_str: &str) -> Result<Vec<WorkspaceData>, ()> {
    match serde_json::from_str(json_str) {
        Ok(val) => return Ok(val),
        Err(_) => return Err(()),
    };
}

fn get_args() -> Vec<String> {
    let tmp_vec: Vec<String> = env::args().collect();
    return tmp_vec;
}

fn calculate_workspaces(mode: ProgMode, workspaces: Vec<WorkspaceData>, min: usize, max: usize) -> Result<usize, ()> {
    for workspace in workspaces {
        if workspace.focused == true {
            let mut num: usize = workspace.num;
            match mode {
                ProgMode::Prev => {
                    if num <= min {
                        return Err(());
                    }
                    num -= 1;
                    return Ok(num);
                }
                ProgMode::Next => {
                    if num >= max {
                        return Err(());
                    }
                    num += 1;
                    return Ok(num);
                }
            }
        }
    }
    return Err(());
}

fn main() {
    let user_args: Vec<String> = get_args();
    let mut min:usize = 1;
    let mut max:usize = 10;
    if user_args.len() < 2 {
        eprintln!("ERR: Not enought args.");
        process::exit(1);
    }
    for idx in 0..user_args.len() {
        match &user_args[idx][..] {
            "-min" => {
                if idx+2 > user_args.len() {
                    process::exit(1);
                }
                min = user_args[idx+1].trim().parse().unwrap_or(1);
            },
            "-max" => {
                if idx+2 > user_args.len() {
                    process::exit(1);
                }
                max = user_args[idx+1].trim().parse().unwrap_or(10);
            },
            _ => {}

        }
    }

    let progs: &str = "swaymsg";
    let args_get_workspaces: Vec<&str> = vec!["-t", "get_workspaces"];
    let mut args_set_workspaces: Vec<&str> = vec!["workspace"];
    let workspace_num: usize;
    let json_output: String = match spawn_process(progs, args_get_workspaces, true) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("ERR: Failed to spawn process {}", progs);
            process::exit(1);
        }
    };
    let json_data: Vec<WorkspaceData> = match parse_json(&json_output) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("ERR: Failed to parse process json output");
            process::exit(1);
        }
    };
    match &user_args[1][..] {
        "p" | "prev" => {
            workspace_num = match calculate_workspaces(ProgMode::Prev, json_data, min, max) {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("ERR: Failed to calculate the prev workspace.");
                    process::exit(1);
                }
            };
        }
        "n" | "next" => {
            workspace_num = match calculate_workspaces(ProgMode::Next, json_data, min, max) {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("ERR: Failed to calculate the next workspace.");
                    process::exit(1);
                }
            };
        }
        _ => process::exit(1)
    }
    let workspace_num_str: &str = &workspace_num.to_string();
    args_set_workspaces.push(workspace_num_str);
    spawn_process(progs, args_set_workspaces, false).unwrap();
}
