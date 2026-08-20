use anyhow::Result;
use clap::Parser;
use memmap2::Mmap;
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use std::fs::File;
const MB: usize = 1024usize.pow(2);
#[derive(Debug, Parser)]
struct Args {
    files: Vec<String>,
    #[arg(short, long)]
    lines: bool,
    #[arg(short, long)]
    words: bool,
    #[arg(short, long)]
    chars: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let all = !args.lines && !args.words && !args.chars;
    for path in args.files.iter() {
        let file = File::open(path)?;
        let map = unsafe { Mmap::map(&file) }?;
        let size = map.len();
        if size < 10 * MB {
            non_parallel(&args, &map);
        } else {
            parallel(&args, &map);
        }
        if args.chars || all {
            print!("{} ", size)
        }
        println!("{}", path);
    }
    Ok(())
}
fn parallel(args: &Args, map: &[u8]) {
    let all = !args.lines && !args.words && !args.chars;
    let base = map.par_split_inclusive(|b| *b == b'\n');
    if args.lines || all {
        print!("{} ", base.clone().count());
    }
    if args.words || all {
        let w = base
            .fold(|| 0, |acc, word| acc + word.split(|b| *b == b' ').count())
            .reduce(|| 0, |acc, count| acc + count);
        print!("{} ", w);
    }
}
fn non_parallel(args: &Args, map: &[u8]) {
    let all = !args.lines && !args.words && !args.chars;
    let base = map.split_inclusive(|b| *b == b'\n');
    if args.lines || all {
        print!("{} ", base.clone().count());
    }
    if args.words || all {
        let w = base.fold(0, |acc, word| acc + word.split(|b| *b == b' ').count());
        print!("{} ", w);
    }
}

