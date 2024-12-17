use crate::runner::RunnerContext;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, process};

#[derive(Debug, Deserialize)]
struct PackageJson {
    scripts: Option<Value>,
    #[serde(rename = "scripts-info")]
    scripts_info: Option<Value>,
    dependencies: Option<Value>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<Value>,
}

fn get_package_json(ctx: Option<RunnerContext>) -> Option<PackageJson> {
    // Get user custom cwd or using current directory
    // example: /home/oxwazz/ni_rs/
    let cwd = ctx
        .unwrap_or_default()
        .cwd
        .unwrap_or_else(|| env::current_dir().unwrap_or(PathBuf::from(".")));

    // path = user directory + package.json
    // example: /home/oxwazz/ni_rs/package.json
    let path = cwd.join("package.json");

    // Early return error
    if !path.is_file() {
        eprintln!("Cannot find package.json");
        process::exit(1)
    }

    // Read package.json file
    let raw = File::open(path).unwrap_or_else(|_| {
        eprintln!("Failed to read package.json");
        process::exit(1)
    });

    // Parse package.json value to `PackageJson` struct
    let data: Option<PackageJson> = serde_json::from_reader(raw).unwrap_or_else(|_| {
        eprintln!("Failed to parse package.json");
        process::exit(1)
    });
    // return
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_package_json_works() {
        let result = get_package_json(Some(RunnerContext {
            cwd: Some(PathBuf::from("./src/test").canonicalize().unwrap()),
            programmatic: None,
            has_lock: None,
        }));
        insta::assert_debug_snapshot!(result);
    }
}
