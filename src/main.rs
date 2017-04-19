extern crate docopt;
extern crate pbr;
extern crate rustc_serialize;

use std::cmp;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::iter;

use docopt::Docopt;
use pbr::ProgressBar;

const USAGE: &'static str = "
Lines

Generate many many lines of \"hi\"

Usage:
  lines <count> <file>
  lines (-h | --help)

Options:
  -h --help     Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_count: usize,
    arg_file: String,
}

const ITERATION_SIZE: usize = 65535;

// TODO: Configure string, configure buffer size
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{} lines to {}", args.arg_count, args.arg_file);

    let payload = b"hi\n";
    let buffer: Vec<u8> = iter::repeat(payload)
        .take(ITERATION_SIZE)
        .cloned()
        .flat_map(|s| s.to_vec())
        .collect();

    let iterations = args.arg_count / ITERATION_SIZE;
    let iterations = if args.arg_count % ITERATION_SIZE > 0 {
        iterations + 1
    } else {
        iterations
    };

    let file = File::create(&args.arg_file).expect("Cannot create file");
    let mut writer = BufWriter::with_capacity(buffer.len(), file);

    let mut pb = ProgressBar::new(args.arg_count as u64);
    pb.format("╢▌▌░╟");

    for iteration in 0..iterations {
        let start_count = iteration * ITERATION_SIZE;
        let end_count = cmp::min(start_count + ITERATION_SIZE, args.arg_count);
        let iteration_count = end_count - start_count;
        let bytes_count = iteration_count * payload.len();

        writer
            .write_all(&buffer[0..bytes_count])
            .expect("Cannot write buffer to file");
        pb.add(iteration_count as u64);
    }

    pb.finish_print("Done");
}

// TODO: Add test
