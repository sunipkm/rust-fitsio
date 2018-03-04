extern crate fitsio;

use std::env;
use std::process;

fn main() {
    let mut nfiles = 0;
    env::args().skip(1).for_each(|arg| {
        if let Ok(_) = fitsio::FitsFile::open(arg).map(|mut f| f.pretty_print()) {
            nfiles += 1;
        }
    });

    if nfiles == 0 {
        eprintln!("No valid fits files supplied");
        process::exit(1);
    }
}
