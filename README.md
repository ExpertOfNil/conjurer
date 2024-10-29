# Conjurer: powerful summoner of new projects and task completion

## Help

For help text, run `summon help` or `summon -h`.  For help on a specific
command, run `summon new -h` or `summon task -h`.

## Tasks

Tasks are defined in a `conjurer.toml` file.  When `summon task <task_name>`
looks for the `conjurer.toml` in the current directory.  If present, it will
match `<task_name>` with a task `name` or `alias` and run the listed ocmmands.
Here is an example that will echo "Parent Dir:", navigate to the parent
directory, and will run the `pwd` command.

```toml
[[task]]
name = "test"
alias = "t"
commands = [
    "echo Parent Dir:",
    "cd .. && pwd",
]
```

a `pre_tasks` string array can be included, which can include other task names
(or aliases) to be run prior to `commands`.  This should help alleviate
repetition.

```toml
[[task]]
name = "test"
alias = "t"
commands = [
    "echo Parent Dir:",
    "cd .. && pwd",
]

[[task]]
name = "other_test"
alias = "ot"
pre_tasks = ["test"]
commands = [
    "ls -al",
]
```

### Available Fields:

* `name` **(required)**: Name of the task
* `commands` **(required)**: List of commands to be run.
    * **NOTE**: all commands are run from `dir` (or the current directory if
        `dir` is omitted)
* `alias` : Alternate (usually abbreviated) name for the task
* `dir` : Directory from which each command in `commands` will be run.
    * Default is the current directory (where `summon` was executed)
* `pre_tasks` : List of task names whose commands are to be run before this
    task's commands

## Project Creation

The `summon new <type> <name>` command provides a couple capabilities:
1. Uses a built-in template for creating a new `conjurer.toml` file in the
    current directory.
1. Uses built-in templates for creating a new project structure (currently
    limited to C++ and Odin) relative to the current directory.

### C++

Running `summon new cpp test_proj` will create a directory `test_proj`, a
`conjurer.toml` file pre-loaded with basic tasks directed at `test_proj`, a
`CMakeLists.txt` (with the project name inserted within `project()`), `src`,
`include`, and `build` folders, and a `main.cpp` file within `src` that contains
some very basic code.

### Odin

Running `summon new odin test_proj` will create a directory `test_proj`,
`odinfmt.json` and `ols.json` files, and a `src` directory with a `main.odin`
that contains some very basic code. A very minimal `conjurer.toml` is included
since the Odin compiler already has built-in build and run executables.

## TODO

- [ ] Python project template
- [ ] Ability to "plug-in" new templates

