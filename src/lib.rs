use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> Config {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Irfan Abliz <irfanabliz914@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("number all output lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("number non-blank output lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Config {
        files: matches
            .get_many("files")
            .expect("No files given")
            .cloned()
            .collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    }
}

pub fn run(config: Config) -> Result<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{last_num:6}\t{line}");
                        } else {
                            println!();
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
