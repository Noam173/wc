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
enum Mode {
    NonParallel,
    Parallel,
}
fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let args = Args::parse();
    let mut mode = Mode::NonParallel;
    let all = !args.lines && !args.words && !args.chars;
    for path in &args.files {
        let file = File::open(path)?;
        let map = unsafe { Mmap::map(&file) }?;
        let size = map.len();
        if size > 10 * MB {
            mode = Mode::Parallel;
        }
        count(&args, &map, &mode);
        if args.chars || all {
            print!("{size} ");
        }
        println!("{path}");
    }
    println!("{:?}", start.elapsed());
    Ok(())
}

fn count(args: &Args, map: &[u8], mode: &Mode) {
    match mode {
        Mode::NonParallel => {
            let base = map.split_inclusive(|b| *b == b'\n');
            let all = !args.lines && !args.words && !args.chars;
            if args.lines || all {
                print!("{} ", base.clone().count());
            }
            if args.words || all {
                let w: usize = base.map(|word| word.split(|b| *b == b' ').count()).sum();
                print!("{w} ");
            }
        }
        Mode::Parallel => {
            let base = map.par_split_inclusive(|b| *b == b'\n');
            let all = !args.lines && !args.words && !args.chars;
            if args.lines || all {
                print!("{} ", base.clone().count());
            }
            if args.words || all {
                let w: usize = base.map(|word| word.split(|b| *b == b' ').count()).sum();
                print!("{w} ");
            }
        }
    }
}
