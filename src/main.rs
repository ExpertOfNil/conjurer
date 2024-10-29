use clap::{Parser, Subcommand};
use log::LevelFilter;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
struct Config {
    task: Vec<Task>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct Task {
    name: String,
    #[serde(default)]
    alias: String,
    commands: Vec<String>,
    #[serde(default)]
    dir: Option<std::path::PathBuf>,
}

#[derive(Parser, Debug)]
struct Args {
    /// Name of task to run, or `new-toml` to create a new conjure.toml.
    name: String,
}

static NEW_TASK: &str = r#"
[[task]]
# Name of the task
name = "test"
# Another name for the task
alias = "t"
# Defaults to the current directory
dir = ""
# Commands to run
commands = [
    "echo 'parent path:'",
    "cd .. && pwd",
]"#;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter(None, LevelFilter::Warn)
        .init();
    let pwd = std::env::current_dir()?.join("conjurer.toml");
    if !pwd.exists() {
        return Err(anyhow::anyhow!(
            "There is not a conjurer.toml in your current path"
        ));
    }

    let tasks: Vec<Task> = toml::from_str::<Config>(&std::fs::read_to_string(pwd)?)?.task;
    let task_map = process_input(tasks)?;
    log::debug!("TASKS: {:?}", task_map);

    let args = Args::parse();
    if args.name == "new-toml" {
        let current_dir = std::env::current_dir()?;
        let toml_file = std::path::Path::new(&current_dir).join("conjurer.toml");
        if toml_file.exists() {
            log::warn!("Backing up existing `conjurer.toml` file to `conjurer.bak.toml`");
            let new_toml_file = std::path::Path::new(&current_dir).join("conjurer.bak.toml");
            std::fs::rename(&toml_file, new_toml_file)?;
        }
        return std::fs::write(toml_file, NEW_TASK).map_err(|e| e.into());
    }
    if let Some(task) = task_map.get(&args.name) {
        for cmd in task.commands.iter() {
            let output = match &task.dir {
                Some(task_dir) => Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .current_dir(task_dir)
                    .stdout(Stdio::inherit())
                    .output()
                    .expect("failed to run command"),
                None => Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .stdout(Stdio::inherit())
                    .output()
                    .expect("failed to run command"),
            };
            match output.status.code() {
                Some(0) => log::info!("EXIT SUCCESS: {}", cmd),
                Some(code) => {
                    log::error!("EXIT FAIL [{}]: {}", code, cmd);
                    return Err(anyhow::anyhow!("Command failed: {}", cmd));
                }
                None => log::warn!("EXIT UNKNOWN: {}", cmd),
            }
        }
    }

    Ok(())
}

fn process_input(tasks: Vec<Task>) -> anyhow::Result<HashMap<String, Task>> {
    let mut task_map = HashMap::new();
    for mut t in tasks.into_iter() {
        if t.name.is_empty() {
            return Err(anyhow::anyhow!("Missing `name` field."));
        }
        if t.commands.is_empty() {
            return Err(anyhow::anyhow!("Missing `commands` for {}.", t.name));
        }
        if let Some(dir) = &mut t.dir {
            if dir.to_str().is_some_and(|p| p.is_empty()) {
                *dir = std::env::current_dir()?;
            }
            if !dir.exists() || !dir.is_dir() {
                return Err(anyhow::anyhow!("Path does not exist: {:?}", dir));
            }
            if !dir.is_dir() {
                return Err(anyhow::anyhow!("Path is not a directory: {:?}", dir));
            }
        }
        if !t.alias.is_empty() {
            _ = task_map.insert(t.alias.clone(), t.clone());
        }
        _ = task_map.insert(t.name.clone(), t);
    }
    Ok(task_map)
}
