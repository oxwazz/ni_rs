use package_manager_detector_rs::commands::{construct_command, ResolveCommandReturn, COMMANDS};

use crate::utils::exclude;

fn get_command(
    agent: &str,
    command: &str,
    args: Vec<&str>,
) -> Result<ResolveCommandReturn, String> {
    let v = match COMMANDS.get(agent) {
        None => return Err(format!("Unsupported agent '{agent}'")),
        Some(v) => v,
    };
    let v = match v.get(command) {
        None => {
            return Err(format!(
                "Command '{command}' is not support by agent '{agent}'"
            ))
        }
        Some(v) => v,
    };
    match construct_command(v, args) {
        None => Err("Unkown error from construct_command".to_string()),
        Some(v) => Ok(v),
    }
}

fn parse_nr(agent: &str, mut args: Vec<&str>) -> Result<ResolveCommandReturn, String> {
    if args.is_empty() {
        args.push("start")
    }
    let mut has_if_present = false;
    if args.contains(&"--if-present") {
        args = exclude(&args, "--if-present");
        has_if_present = true;
    }
    let mut cmd = get_command(agent, "run", args)?;
    if has_if_present {
        cmd.args.insert(1, "--if-present".to_string());
    }
    Ok(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    // fn get_command
    #[test]
    fn npm_install() {
        let output = get_command("npm", "run", vec!["axios"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "npm".to_string(),
                args: vec!["run".to_string(), "axios".to_string()],
            }),
        );
    }
    #[test]
    fn yarn_install() {
        let output = get_command("yarn", "run", vec!["axios"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec!["run".to_string(), "axios".to_string()],
            }),
        );
    }
    #[test]
    fn not_supported_agent() {
        let output = get_command("xxx", "run", vec!["axios"]);
        assert_eq!(output, Err("Unsupported agent 'xxx'".to_string()));
    }
    #[test]
    fn not_supported_command() {
        let output = get_command("npm", "xxx", vec!["axios"]);
        assert_eq!(
            output,
            Err("Command 'xxx' is not support by agent 'npm'".to_string())
        );
    }

    // parse_nr - bun.spec.ts
    #[test]
    fn bun_empty() {
        let output = parse_nr("bun", vec![]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "bun".to_string(),
                args: vec!["run".to_string(), "start".to_string()],
            }),
        );
    }
    #[test]
    fn bun_script() {
        let output = parse_nr("bun", vec!["dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "bun".to_string(),
                args: vec!["run".to_string(), "dev".to_string()],
            }),
        );
    }
    #[test]
    fn bun_script_with_arguments() {
        let output = parse_nr("bun", vec!["build", "--watch", "-o"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "bun".to_string(),
                args: vec![
                    "run".to_string(),
                    "build".to_string(),
                    "--watch".to_string(),
                    "-o".to_string()
                ],
            }),
        );
    }
    #[test]
    fn bun_colon() {
        let output = parse_nr("bun", vec!["build:dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "bun".to_string(),
                args: vec!["run".to_string(), "build:dev".to_string()],
            }),
        );
    }
    #[test]
    fn bun_if_present() {
        let output = parse_nr("bun", vec!["test", "--if-present"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "bun".to_string(),
                args: vec![
                    "run".to_string(),
                    "--if-present".to_string(),
                    "test".to_string()
                ],
            }),
        );
    }

    // parse_nr - npm.spec.ts
    #[test]
    fn npm_empty() {
        let output = parse_nr("npm", vec![]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "npm".to_string(),
                args: vec!["run".to_string(), "start".to_string()],
            }),
        );
    }
    #[test]
    fn npm_script() {
        let output = parse_nr("npm", vec!["dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "npm".to_string(),
                args: vec!["run".to_string(), "dev".to_string()],
            }),
        );
    }
    #[test]
    fn npm_script_with_arguments() {
        let output = parse_nr("npm", vec!["build", "--watch", "-o"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "npm".to_string(),
                args: vec![
                    "run".to_string(),
                    "build".to_string(),
                    "--".to_string(),
                    "--watch".to_string(),
                    "-o".to_string()
                ],
            }),
        );
    }
    #[test]
    fn npm_colon() {
        let output = parse_nr("npm", vec!["build:dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "npm".to_string(),
                args: vec!["run".to_string(), "build:dev".to_string()],
            }),
        );
    }
    #[test]
    fn npm_if_present() {
        let output = parse_nr("npm", vec!["test", "--if-present"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "npm".to_string(),
                args: vec![
                    "run".to_string(),
                    "--if-present".to_string(),
                    "test".to_string()
                ],
            }),
        );
    }

    // parse_nr - pnpm.spec.ts
    #[test]
    fn pnpm_empty() {
        let output = parse_nr("pnpm", vec![]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "pnpm".to_string(),
                args: vec!["run".to_string(), "start".to_string()],
            }),
        );
    }
    #[test]
    fn pnpm_script() {
        let output = parse_nr("pnpm", vec!["dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "pnpm".to_string(),
                args: vec!["run".to_string(), "dev".to_string()],
            }),
        );
    }
    #[test]
    fn pnpm_script_with_arguments() {
        let output = parse_nr("pnpm", vec!["build", "--watch", "-o"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "pnpm".to_string(),
                args: vec![
                    "run".to_string(),
                    "build".to_string(),
                    "--watch".to_string(),
                    "-o".to_string()
                ],
            }),
        );
    }
    #[test]
    fn pnpm_colon() {
        let output = parse_nr("pnpm", vec!["build:dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "pnpm".to_string(),
                args: vec!["run".to_string(), "build:dev".to_string()],
            }),
        );
    }
    #[test]
    fn pnpm_if_present() {
        let output = parse_nr("pnpm", vec!["test", "--if-present"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "pnpm".to_string(),
                args: vec![
                    "run".to_string(),
                    "--if-present".to_string(),
                    "test".to_string()
                ],
            }),
        );
    }

    // parse_nr - yarn.spec.ts
    #[test]
    fn yarn_empty() {
        let output = parse_nr("yarn", vec![]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec!["run".to_string(), "start".to_string()],
            }),
        );
    }
    #[test]
    fn yarn_script() {
        let output = parse_nr("yarn", vec!["dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec!["run".to_string(), "dev".to_string()],
            }),
        );
    }
    #[test]
    fn yarn_script_with_arguments() {
        let output = parse_nr("yarn", vec!["build", "--watch", "-o"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec![
                    "run".to_string(),
                    "build".to_string(),
                    "--watch".to_string(),
                    "-o".to_string()
                ],
            }),
        );
    }
    #[test]
    fn yarn_colon() {
        let output = parse_nr("yarn", vec!["build:dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec!["run".to_string(), "build:dev".to_string()],
            }),
        );
    }
    #[test]
    fn yarn_if_present() {
        let output = parse_nr("yarn", vec!["test", "--if-present"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec![
                    "run".to_string(),
                    "--if-present".to_string(),
                    "test".to_string()
                ],
            }),
        );
    }

    // parse_nr - yarn@berry.spec.ts
    #[test]
    fn yarn_berry_empty() {
        let output = parse_nr("yarn", vec![]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec!["run".to_string(), "start".to_string()],
            }),
        );
    }
    #[test]
    fn yarn_berry_script() {
        let output = parse_nr("yarn", vec!["dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec!["run".to_string(), "dev".to_string()],
            }),
        );
    }
    #[test]
    fn yarn_berry_script_with_arguments() {
        let output = parse_nr("yarn", vec!["build", "--watch", "-o"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec![
                    "run".to_string(),
                    "build".to_string(),
                    "--watch".to_string(),
                    "-o".to_string()
                ],
            }),
        );
    }
    #[test]
    fn yarn_berry_colon() {
        let output = parse_nr("yarn", vec!["build:dev"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec!["run".to_string(), "build:dev".to_string()],
            }),
        );
    }
    #[test]
    fn yarn_berry_if_present() {
        let output = parse_nr("yarn", vec!["test", "--if-present"]);
        assert_eq!(
            output,
            Ok(ResolveCommandReturn {
                command: "yarn".to_string(),
                args: vec![
                    "run".to_string(),
                    "--if-present".to_string(),
                    "test".to_string()
                ],
            }),
        );
    }
}
