mod templates;

use clap::{Args, Parser, Subcommand, ValueEnum};
use log::LevelFilter;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use templates::*;

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
    #[serde(default)]
    pre_tasks: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        #[command(subcommand)]
        new_type: NewCommandType,
    },
    Task {
        /// Name of task to run, or `new-toml` to create a new conjure.toml.
        name: String,
    },
}

#[derive(Clone, Debug, Subcommand)]
enum NewCommandType {
    /// Create a new C++ project from template.
    Cpp {
        /// Project name
        name: String,
    },
    /// Create a new Odin project from template.
    Odin {
        /// Project name
        name: String,
    },
    /// Create a new `conjure.toml` from template.
    Toml,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Cli::parse();
    match args.command {
        Commands::New { new_type } => match new_type {
            NewCommandType::Toml => {
                create_new_toml()?;
            }
            NewCommandType::Cpp { name, .. } => {
                create_cpp_project(&name)?;
            }
            NewCommandType::Odin { name, .. } => {
                println!("Make a Odin project: {:?}", name);
                unimplemented!();
            }
        },
        Commands::Task { name } => {
            let pwd = std::env::current_dir()?.join("conjurer.toml");
            if !pwd.exists() {
                return Err(anyhow::anyhow!(
                    "There is not a conjurer.toml in your current path"
                ));
            }

            let tasks: Vec<Task> = toml::from_str::<Config>(&std::fs::read_to_string(pwd)?)?.task;
            let task_map = process_input(tasks)?;
            log::debug!("TASKS: {:?}", task_map);
            if let Some(task) = task_map.get(&name) {
                if let Some(pre_tasks) = &task.pre_tasks {
                    for task_key in pre_tasks {
                        let pre_task = task_map
                            .get(task_key)
                            .ok_or(anyhow::anyhow!("Pre-task {} not found.", task_key))?;
                        run_commands(pre_task)?;
                    }
                }
                run_commands(task)?;
            }
        }
    };
    Ok(())
}

fn run_commands(task: &Task) -> anyhow::Result<()> {
    for cmd in task.commands.iter() {
        log::debug!("Running command: {}", cmd);
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
