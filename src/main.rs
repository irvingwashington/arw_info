extern crate getopts;
#[macro_use]
extern crate lazy_static;
extern crate num;

use getopts::Options;
use std::env;
use std::fs::File;
mod arw_file;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} ARW_FILE [options]", program);
    println!("arw_info {}\n", VERSION);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    match File::open(&input) {
        Err(err) => {
            println!("{:?}", err);
            return;
        }
        Ok(_) => {}
    }
    arw_file::info(&input);
}
