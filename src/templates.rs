use std::fs;
use std::path::Path;

static CONJURER_TOML: &str = include_str!("../templates/conjurer.toml");

pub(crate) fn create_new_toml() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let toml_file = std::path::Path::new(&cwd).join("conjurer.toml");
    if toml_file.exists() {
        log::warn!("Backing up existing `conjurer.toml` file to `conjurer.bak.toml`");
        let new_toml_file = std::path::Path::new(&cwd).join("conjurer.bak.toml");
        std::fs::rename(&toml_file, new_toml_file)?;
    }
    std::fs::write(toml_file, CONJURER_TOML).map_err(|e| e.into())
}

pub (crate) fn create_cpp_toml(name: &str) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let conjure = Path::new(&cwd).join("conjurer.toml");
    let project_root_str = cwd.to_str().expect("Could not create string from `cwd`");
    let conjure_content = String::from(CONJURER_CPP)
        .replace("CONJURER_PROJECT_DIR", project_root_str)
        .replace("CONJURER_PROJECT_NAME", name);
    fs::write(conjure, conjure_content)?;
    Ok(())
}

pub (crate) fn create_odin_toml() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let conjure = Path::new(&cwd).join("conjurer.toml");
    fs::write(conjure, CONJURER_ODIN)?;
    Ok(())
}

static MAIN_CPP: &str = include_str!("../templates/cpp/main.cpp");
static CMAKELISTS: &str = include_str!("../templates/cpp/CMakeLists.txt");
static CONJURER_CPP: &str = include_str!("../templates/cpp/conjurer.toml");

pub(crate) fn create_cpp_project(name: &str) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let proj_root = Path::new(&cwd).join(name);
    fs::create_dir(&proj_root)?;
    let cwd = proj_root;
    fs::create_dir(cwd.join("build"))?;
    fs::create_dir(cwd.join("include"))?;

    let cmakelists = Path::new(&cwd).join("CMakeLists.txt");
    let cmakelists_content = String::from(CMAKELISTS).replace("CONJURER_PROJECT_NAME", name);
    fs::write(cmakelists, cmakelists_content)?;

    let conjure = Path::new(&cwd).join("conjurer.toml");
    let project_root_str = cwd.to_str().expect("Could not create string from `cwd`");
    let conjure_content = String::from(CONJURER_CPP)
        .replace("CONJURER_PROJECT_DIR", project_root_str)
        .replace("CONJURER_PROJECT_NAME", name);
    fs::write(conjure, conjure_content)?;

    let src_dir = Path::new(&cwd).join("src");
    fs::create_dir(&src_dir)?;
    let cwd = src_dir;
    let main_cpp = Path::new(&cwd).join("main.cpp");
    fs::write(main_cpp, MAIN_CPP)?;
    Ok(())
}

static MAIN_ODIN: &str = include_str!("../templates/odin/main.odin");
static ODINFMT: &str = include_str!("../templates/odin/odinfmt.json");
static OLS: &str = include_str!("../templates/odin/ols.json");
static CONJURER_ODIN: &str = include_str!("../templates/odin/conjurer.toml");

pub(crate) fn create_odin_project(name: &str) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let proj_root = Path::new(&cwd).join(name);
    fs::create_dir(&proj_root)?;
    let cwd = proj_root;

    let odinfmt = Path::new(&cwd).join("odinfmt.json");
    fs::write(odinfmt, ODINFMT)?;

    let ols = Path::new(&cwd).join("ols.json");
    fs::write(ols, OLS)?;

    let conjure = Path::new(&cwd).join("conjurer.toml");
    fs::write(conjure, CONJURER_ODIN)?;

    let src_dir = Path::new(&cwd).join("src");
    fs::create_dir(&src_dir)?;
    let cwd = src_dir;
    let main_cpp = Path::new(&cwd).join("main.odin");
    fs::write(main_cpp, MAIN_ODIN)?;
    Ok(())
}
