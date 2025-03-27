use anyhow::Context;
use clap::*;
use serde_json::Value;
use std::process::Command;

/// Create clap subcommand arguments
pub fn make_subcommand() -> clap::Command {
    clap::Command::new("test")
        .about("Execute test cases defined in package configuration files")
        .after_help(
            r###"
Execute test cases defined in package configuration files.

Usage:
  cbp build test <PACKAGES>...     # Test specified packages

Test cases are defined in package configuration files under the "tests" section:
  {
    "tests": [
      {
        "name": "test name",       # Optional, defaults to "test #N"
        "command": "command",      # Required, command to execute
        "args": ["arg1", "arg2"],  # Optional, command arguments
        "pattern": "regex",        # Optional, pattern to match in output
        "ignore_exit_code": false  # Optional, ignore command exit code
      }
    ]
  }

"###,
        )
        .arg(
            Arg::new("packages")
                .help("Package names to test")
                .required(true)
                .num_args(1..)
                .value_name("PACKAGES"),
        )
        .arg(
            Arg::new("base")
                .long("base")
                .help("Base directory containing packages/ and binaries/")
                .num_args(1)
                .value_name("BASE")
                .default_value("."),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .num_args(1)
                .value_name("DIR")
                .help("Change working directory")
                .hide(true),
        )
}

/// Execute test subcommand
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let base_dir = std::path::PathBuf::from(args.get_one::<String>("base").unwrap());
    let cbp_dirs = if args.contains_id("dir") {
        let home =
            std::path::Path::new(args.get_one::<String>("dir").unwrap()).to_path_buf();
        cbp::CbpDirs::from(home)?
    } else {
        cbp::CbpDirs::from_exe()?
    };

    //----------------------------
    // Operating
    //----------------------------
    let packages: Vec<String> = args
        .get_many::<String>("packages")
        .unwrap()
        .cloned()
        .collect();

    let mut has_error = false;
    for pkg in packages {
        let json = cbp::read_package_json(&base_dir, &pkg)?;

        // Check if tests are defined
        let tests = json.get("tests").and_then(Value::as_array);
        if tests.is_none() {
            println!("Warning: No test cases defined for package {}", pkg);
            continue;
        }

        println!("==> Testing package: {}\n", pkg);

        // Execute test cases
        for (i, test) in tests.unwrap().iter().enumerate() {
            let name = test
                .get("name")
                .and_then(Value::as_str)
                .map(String::from)
                .unwrap_or(format!("test #{}", i));

            let cmd = test
                .get("command")
                .and_then(Value::as_str)
                .context("Missing 'command' field in test case")?;

            let ignore_exit_code = test
                .get("ignore_exit_code")
                .and_then(Value::as_bool)
                .unwrap_or(false);

            let pattern = test.get("pattern").and_then(Value::as_str);

            print!("==> Running test '{}'... ", name);

            // Prepare command
            let full_cmd = if [
                "ls", "cat", "grep", "find", "which", "test", "mkdir", "rm", "cp", "mv",
            ]
            .contains(&cmd)
            {
                cmd.to_string()
            } else {
                cbp_dirs.bin.join(cmd).to_string_lossy().to_string()
            };

            // Get arguments
            let args = test
                .get("args")
                .map(|a| match a {
                    Value::String(s) => {
                        // Handle command substitution in string
                        if s.contains("$(cbp prefix)") {
                            vec![s.replace(
                                "$(cbp prefix)",
                                &cbp_dirs.home.to_string_lossy(),
                            )]
                        } else {
                            vec![s.clone()]
                        }
                    }
                    Value::Array(arr) => arr
                        .iter()
                        .filter_map(Value::as_str)
                        .map(|s| {
                            // Handle command substitution in array elements
                            if s.contains("$(cbp prefix)") {
                                s.replace(
                                    "$(cbp prefix)",
                                    &cbp_dirs.home.to_string_lossy(),
                                )
                            } else {
                                s.to_string()
                            }
                        })
                        .collect(),
                    _ => Vec::new(),
                })
                .unwrap_or_default();

            // Execute command
            let output = Command::new(&full_cmd)
                .args(&args)
                .output()
                .with_context(|| format!("Failed to execute command: {}", full_cmd))?;

            let output_str = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();
            let combined_output = format!("{}{}", output_str, stderr_str);

            // Check results
            if !output.status.success() && !ignore_exit_code {
                println!("FAILED");
                println!(
                    "  - Command failed with exit code {}",
                    output.status.code().unwrap_or(-1)
                );
                println!("  - Output: {}", combined_output);
                has_error = true;
                continue;
            }

            if let Some(pattern) = pattern {
                // Use (?m) to enable multiline mode, making $ match end of each line
                let pattern = format!("(?m){}", pattern);
                if regex::Regex::new(&pattern)?.is_match(&combined_output) {
                    println!("PASSED");
                } else {
                    println!("FAILED");
                    println!("  - Output does not match pattern: {}", pattern);
                    println!("  - Output: {}", combined_output);
                    has_error = true;
                }
            } else {
                println!("PASSED");
            }
        }
    }

    if has_error {
        std::process::exit(1);
    }

    Ok(())
}
