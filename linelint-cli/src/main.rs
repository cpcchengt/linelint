use clap::{Arg, Command};
use linelint::config::Config;
use linelint::line::LineEnding;
use linelint::linter::Linter;
use linelint::rule::line_end_lint::LineEndLint;
use linelint::rule::trailing_ws_lint::TrailingWhitespaceLint;
use std::path::{Path, PathBuf};

fn check(current_dir: &Path, exclude_paths: Vec<PathBuf>, linter: &Linter) {
    match linter.check_files_in_dir(current_dir, exclude_paths) {
        Ok(issues) => {
            if issues.is_empty() {
                println!("No issues found.");
            } else {
                for issue in issues {
                    println!(
                        "{}: {} in file {} at line {}",
                        issue.rule, issue.description, issue.filename, issue.line_number
                    );
                }
                std::process::exit(2);
            }
        }
        Err(errors) => {
            for e in errors {
                eprintln!("Error checking files: {:?}", e);
            }
        }
    }
}

fn format(current_dir: &Path, exclude_paths: Vec<PathBuf>, linter: &Linter) {
    match linter.format_files_in_dir(current_dir, exclude_paths) {
        Ok(_) => println!("Files formatted successfully."),
        Err(errors) => {
            for e in errors {
                eprintln!("Error formatting files: {:?}", e);
            }
        }
    }
}

fn main() {
    let matches = Command::new("linelint-cli")
        .version("0.0.2")
        .about("A command-line tool for linting and fixing line formatting issues")
        .subcommand(
            Command::new("check")
                .about("Check files for lint issues")
                .arg(
                    Arg::new("exclude")
                        .long("exclude")
                        .short('e')
                        .help("Exclude files or directories for check"),
                ),
        )
        .subcommand(
            Command::new("format")
                .about("Automatically format files to fix lint issues")
                .arg(
                    Arg::new("exclude")
                        .long("exclude")
                        .short('e')
                        .help("Exclude files or directories for check"),
                ),
        )
        .get_matches();

    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let mut config = Config::new(LineEnding::Auto);
    config.add_rule(Box::new(LineEndLint {}));
    config.add_rule(Box::new(TrailingWhitespaceLint {}));

    let linter = Linter::new(&config);

    if matches.subcommand().is_none() {
        println!("No subcommand provided, defaulting to 'check'...");
        check(&current_dir, vec![], &linter);
    } else if let Some(check_match) = matches.subcommand_matches("check") {
        let ex = check_match.get_one::<String>("exclude");
        let exclude_paths: Vec<PathBuf> = if let Some(ex) = ex {
            ex.split(',').map(|s| current_dir.join(s)).collect()
        } else {
            vec![]
        };
        check(&current_dir, exclude_paths, &linter);
    } else if let Some(format_match) = matches.subcommand_matches("format") {
        let ex = format_match.get_one::<String>("exclude");
        let exclude_paths: Vec<PathBuf> = if let Some(ex) = ex {
            ex.split(',').map(|s| current_dir.join(s)).collect()
        } else {
            vec![]
        };
        format(&current_dir, exclude_paths, &linter);
    }
}
