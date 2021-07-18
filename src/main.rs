// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use log::error;
use req::Value;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Arguments {
    #[structopt(
        short = "q",
        long = "quiet",
        help = "Quiet mode",
        conflicts_with = "verbose"
    )]
    quiet: bool,

    #[structopt(
        short = "v",
        long = "verbose",
        parse(from_occurrences),
        conflicts_with = "quiet",
        help = "Verbose mode"
    )]
    verbose: usize,

    #[structopt(
        short = "f",
        long = "file",
        parse(from_os_str),
        help = "Processed file (default: read from STDIN)"
    )]
    file: Option<PathBuf>,

    #[structopt(name = "REGEX", help = "The regex pattern")]
    regex: String,
}

fn main() {
    let args = Arguments::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbose)
        .init()
        .unwrap();

    match Value::new(args.file, &args.regex) {
        Ok(_) => {}
        Err(err) => error!("{}", err),
    }
}

// vim: set ts=4 sw=4 expandtab:
