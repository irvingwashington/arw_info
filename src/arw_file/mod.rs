use std::io;
use std::fs::File;

mod header;

pub fn info(filename: &str) {
    let mut file_handle = try!(File::open(filename));
    let mut header = header::Header::new(&file_handle);
}
