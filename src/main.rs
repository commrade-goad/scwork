use serde_derive::{Deserialize, Serialize};
use std::env;
use std::process;
use std::process::{Command, Output};

#[derive(Deserialize, Serialize)]
struct WorkspaceData {
    name: String,
    focused: bool,
}

enum ProgMode {
    Prev,
    Next,
}

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

fn calculate_workspaces(mode: ProgMode, workspaces: Vec<WorkspaceData>) -> Result<usize, ()> {
    for workspace in workspaces {
        if workspace.focused == true {
            let mut num: usize = match workspace.name.trim().parse() {
                Ok(val) => val,
                Err(_) => return Err(()),
            };
            match mode {
                ProgMode::Prev => {
                    if num == 1 {
                        return Err(());
                    }
                    num -= 1;
                    return Ok(num);
                }
                ProgMode::Next => {
                    if num == 10 {
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
    if user_args.len() < 2 {
        eprintln!("ERR: Not enought args.");
        process::exit(1);
    }

    let progs: &str = "swaymsg";
    let args_get_workspaces: Vec<&str> = vec!["-t", "get_workspaces"];
    let mut args_set_workspaces: Vec<&str> = vec!["workspace"];
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
            let workspace_num: usize = match calculate_workspaces(ProgMode::Prev, json_data) {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("ERR: Failed to calculate the prev workspace.");
                    process::exit(1);
                }
            };
            let workspace_num_str: &str = &workspace_num.to_string();
            args_set_workspaces.push(workspace_num_str);
            spawn_process(progs, args_set_workspaces, false).unwrap();
        }
        "n" | "next" => {
            let workspace_num: usize = match calculate_workspaces(ProgMode::Next, json_data) {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("ERR: Failed to calculate the next workspace.");
                    process::exit(1);
                }
            };
            let workspace_num_str: &str = &workspace_num.to_string();
            args_set_workspaces.push(workspace_num_str);
            spawn_process(progs, args_set_workspaces, false).unwrap();
        }
        _ => {}
    }
}
