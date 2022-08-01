#![doc = include_str!("../README.md")]

use anyhow::anyhow;
use anyhow::ensure;
use anyhow::Result;
use std::borrow::Cow;
use std::env;
use std::process;
use std::process::Command;

struct FileTarget {
    line: Option<u32>,
    column: Option<u32>,
    filename: String,
}

struct Args {
    wait: bool,
    file_targets: Vec<FileTarget>,
}

fn parse_args() -> Result<Args> {
    // True if the subprocess should be forked.
    let mut wait = false;
    // A list of all file targets to open.
    let mut file_targets = Vec::new();
    // True if we are parsing the list of filenames and can no longer accept
    // options.
    let mut at_filename_list = false;
    // Line to open the next file.
    let mut line = None;
    // Column to open the next file.
    let mut column = None;

    // Parse the arguments, building a list of file targets.
    let mut it = env::args().into_iter().skip(1);
    loop {
        let arg = match it.next() {
            Some(arg) => arg,
            None => break,
        };
        if !at_filename_list {
            match arg.as_str() {
                // Ignore these options.
                "nosplash" | "dontReopenProjects" | "disableNonBundledPlugins" => continue,
                "--wait" => {
                    wait = true;
                    continue;
                }
                // These aren't supported and probably never will be.
                "diff" | "merge" | "attach-to-process" => {
                    return Err(anyhow!("Unsupported command: {arg}"))
                }
                _ => at_filename_list = true,
            };
        }
        match arg.as_str() {
            "--line" | "-l" => {
                line = match it.next() {
                    Some(arg) => u32::try_from(arg.parse::<i32>()?).ok(),
                    None => return Err(anyhow!("No integer argument passed to {arg}")),
                };
            }
            "--column" | "-c" => {
                column = match it.next() {
                    // The Emacs command-line treats column 1 as the first column.
                    Some(arg) => u32::try_from(arg.parse::<i32>()?).ok().map(|x| x + 1),
                    None => return Err(anyhow!("No integer argument passed to {arg}")),
                };
            }
            _ => {
                file_targets.push(FileTarget {
                    line,
                    column,
                    filename: arg,
                });
                line = None;
                column = None;
            }
        }
    }

    if file_targets.len() != 1 {
        // If more than one file is passed, remove all sln files.
        file_targets.retain(|x| !x.filename.to_ascii_lowercase().ends_with(".sln"));
    }

    Ok(Args { wait, file_targets })
}

fn try_main() -> Result<()> {
    println!(
        "{}",
        env::args()
            .into_iter()
            .map(|x| format!("“{x}”"))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let options = parse_args()?;

    ensure!(
        !options.file_targets.is_empty(),
        "No file arguments provided"
    );

    let mut args = Vec::new();
    if !options.wait {
        args.push(String::from("-n"));
    }
    for file_target in options.file_targets {
        match file_target.column {
            Some(column) => args.push(format!("+{}:{}", file_target.line.unwrap_or(1), column)),
            None => match file_target.line {
                Some(line) => args.push(format!("+{}", line)),
                None => {}
            },
        }
        args.push(file_target.filename);
    }

    // Spawn process.
    let status = if cfg!(target_os = "windows") {
        Command::new("emacsclientw").args(&args).status()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "emacsclient {}",
                args.iter()
                    .map(|x| shell_escape::unix::escape(Cow::Borrowed(x)))
                    .collect::<Vec<_>>()
                    .join(" ")
            ))
            .status()
    }?;

    if !status.success() {
        match status.code() {
            Some(code) => return Err(anyhow!("emacsclient error: {code}")),
            None => return Err(anyhow!("emacsclient error")),
        }
    }

    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    }
}
