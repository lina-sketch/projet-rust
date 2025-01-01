use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::path::Path;
use std::process::Command;
use std::io::BufRead;

pub fn echo(args: Vec<String>) {
    if args.len() > 1 {
        println!("{}", args[1..].join(" "));
    }
}

pub fn cat(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

pub fn ls(path: &str) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}

pub fn grep(pattern: &str, file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}: {}", index + 1, line);
        }
    }
    Ok(())
}

pub fn touch(file_path: &str) -> io::Result<()> {
    File::create(file_path)?;
    println!("File '{}' created or updated.", file_path);
    Ok(())
}

pub fn mkdir(dir_name: &str) -> io::Result<()> {
    fs::create_dir_all(dir_name)?;
    println!("Directory '{}' created.", dir_name);
    Ok(())
}

pub fn head(file_path: &str, lines: usize) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if index >= lines {
            break;
        }
        println!("{}", line?);
    }
    Ok(())
}

pub fn tail(file_path: &str, lines: usize) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let all_lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let start = if all_lines.len() > lines {
        all_lines.len() - lines
    } else {
        0
    };

    for line in &all_lines[start..] {
        println!("{}", line);
    }
    Ok(())
}

fn print_help() {
    println!("Usage: cli_utils <command> [arguments]");
    println!("Commands:");
    println!("  echo   [text]       Print the given text");
    println!("  cat    [file]       Display the contents of a file");
    println!("  ls     [path]       List directory contents");
    println!("  grep   [pattern] [file]  Search for a pattern in a file");
    println!("  touch  [file]       Create or update a file");
    println!("  mkdir  [dir]        Create a directory");
    println!("  head   [file] [n]   Display the first n lines of a file");
    println!("  tail   [file] [n]   Display the last n lines of a file");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "echo" => echo(args),
        "cat" => {
            if args.len() > 2 {
                if let Err(e) = cat(&args[2]) {
                    eprintln!("Error: {}", e);
                }
            } else {
                eprintln!("Usage: cat <file>");
            }
        }
        "ls" => {
            let path = if args.len() > 2 { &args[2] } else { "." };
            if let Err(e) = ls(path) {
                eprintln!("Error: {}", e);
            }
        }
        "grep" => {
            if args.len() > 3 {
                if let Err(e) = grep(&args[2], &args[3]) {
                    eprintln!("Error: {}", e);
                }
            } else {
                eprintln!("Usage: grep <pattern> <file>");
            }
        }
        "touch" => {
            if args.len() > 2 {
                if let Err(e) = touch(&args[2]) {
                    eprintln!("Error: {}", e);
                }
            } else {
                eprintln!("Usage: touch <file>");
            }
        }
        "mkdir" => {
            if args.len() > 2 {
                if let Err(e) = mkdir(&args[2]) {
                    eprintln!("Error: {}", e);
                }
            } else {
                eprintln!("Usage: mkdir <dir>");
            }
        }
        "head" => {
            if args.len() > 3 {
                if let Err(e) = head(&args[2], args[3].parse().unwrap_or(10)) {
                    eprintln!("Error: {}", e);
                }
            } else {
                eprintln!("Usage: head <file> <lines>");
            }
        }
        "tail" => {
            if args.len() > 3 {
                if let Err(e) = tail(&args[2], args[3].parse().unwrap_or(10)) {
                    eprintln!("Error: {}", e);
                }
            } else {
                eprintln!("Usage: tail <file> <lines>");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}
