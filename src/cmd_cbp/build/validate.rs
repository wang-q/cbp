use clap::*;
use jsonschema::JSONSchema;
use serde_json::Value;
use std::path::Path;

/// Create clap subcommand arguments
pub fn make_subcommand() -> clap::Command {
    clap::Command::new("validate")
        .about("Validate package configuration files")
        .arg(
            Arg::new("packages")
                .help("Package names to validate")
                .required(true)
                .num_args(1..)
                .value_name("PACKAGES"),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .help("Base directory containing packages/ and schema/")
                .num_args(1)
                .value_name("DIR")
                .default_value("."),
        )
}

/// Execute validate subcommand
pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    //----------------------------
    // Args
    //----------------------------
    let base_dir = std::path::PathBuf::from(args.get_one::<String>("dir").unwrap());

    //----------------------------
    // Load schema
    //----------------------------
    let schema_str = include_str!("../../../doc/schema/schema.json");
    let schema: Value = serde_json::from_str(schema_str)?;
    let compiled = JSONSchema::compile(&schema)
        .map_err(|e| anyhow::anyhow!("Schema compilation error: {}", e))?;

    //----------------------------
    // Validate packages
    //----------------------------
    let packages: Vec<String> = args
        .get_many::<String>("packages")
        .unwrap()
        .cloned()
        .collect();

    let mut has_error = false;
    for pkg in packages {
        print!("Validating {}... ", pkg);

        let pkg_path = base_dir.join("packages").join(format!("{}.json", pkg));
        if !pkg_path.exists() {
            println!("FAILED");
            println!("  - Package file does not exist");
            has_error = true;
            continue;
        }

        match validate_package(&base_dir, &pkg, &compiled) {
            Ok(_) => println!("PASSED"),
            Err(e) => {
                println!("FAILED");
                println!("  - {}", e);
                has_error = true;
            }
        }
    }

    if has_error {
        std::process::exit(1);
    }

    Ok(())
}

fn validate_package(
    base_dir: &Path,
    pkg: &str,
    schema: &JSONSchema,
) -> anyhow::Result<()> {
    let json = cbp::read_package_json(base_dir, pkg)?;

    schema.validate(&json).map_err(|errors| {
        let error_messages: Vec<String> = errors.map(|e| format!("{}", e)).collect();
        anyhow::anyhow!("{}", error_messages.join("\n  - "))
    })?;

    Ok(())
}
