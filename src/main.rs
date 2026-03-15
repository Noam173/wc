use clap::Parser;
use std::io::{IsTerminal, stdin};
use std::process::exit;
use std::{fs::read_to_string, io::Read};
#[derive(Debug, Parser)]
struct Args {
    /// pass input as pipline or specify a file(s) path
    files: Vec<String>,
    /// count words
    #[arg(short, long)]
    words: bool,
    /// count lines
    #[arg(short, long)]
    lines: bool,
    /// count chars
    #[arg(short, long)]
    chars: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.files.is_empty() {
        tty_input(&args)?;
    } else {
        file_path(&args)?;
    };
    Ok(())
}
fn tty_input(args: &Args) -> std::io::Result<()> {
    let mut file = String::new();
    if !stdin().is_terminal() {
        stdin().read_to_string(&mut file)?;
        count(&file, &args);
    } else {
        exit(1);
    };
    Ok(())
}
fn file_path(args: &Args) -> std::io::Result<()> {
    let (mut lines, mut words, mut chars) = (0, 0, 0);
    for i in &args.files {
        let file = read_to_string(i)?;
        println!("== {} ==", i);
        let (l, w, c) = count(&file, &args);
        lines += l;
        words += w;
        chars += c;
    }
    if args.files.len() != 1 {
        let show_all = !args.words && !args.lines && !args.chars;
        println!("== TOTAL ==");
        if args.lines || show_all {
            println!("Total lines: {}", lines);
        }
        if args.words || show_all {
            println!("Total words: {}", words);
        }
        if args.chars || show_all {
            println!("Total chars: {}", chars);
        }
    };
    Ok(())
}
fn count(file: &str, args: &Args) -> (usize, usize, usize) {
    let lines = file.lines().count();
    let words = file.split_ascii_whitespace().count();
    let chars = file.chars().count();

    let show_all = !args.words && !args.lines && !args.chars;

    if args.lines || show_all {
        println!("lines: {}", lines);
    }
    if args.words || show_all {
        println!("words: {}", words);
    }
    if args.chars || show_all {
        println!("chars: {}", chars);
    }
    (lines, words, chars)
}
