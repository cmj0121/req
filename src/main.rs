// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use log::{error, trace};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct ReqCommandLineTool {
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

    #[structopt(short = "s", long = "single", help = "Find the first matched.")]
    single: bool,

    #[structopt(name = "REGEX", help = "The regex pattern")]
    regex: String,

    #[structopt(
        short = "f",
        long = "file",
        parse(from_os_str),
        help = "Processed file, default from stdin"
    )]
    file: Option<PathBuf>,
}

fn main() {
    let opt = ReqCommandLineTool::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .init()
        .unwrap();

    let text: String = match opt.file {
        None => {
            let mut text = String::new();

            match io::stdin().read_to_string(&mut text) {
                Ok(_) => {}
                Err(err) => {
                    error!("cannot read from STDIN: {}", err);
                    return;
                }
            };
            text
        }
        Some(filename) => match fs::read_to_string(filename.clone()) {
            Ok(text) => text,
            Err(err) => {
                error!("cannot read file {:?}: {}", filename, err);
                return;
            }
        },
    };
    trace!("parse text: {}", text);
}

// vim: set ts=4 sw=4 expandtab:
