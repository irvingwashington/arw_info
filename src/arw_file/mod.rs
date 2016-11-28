use std::fs::File;

mod header;
mod ifd;
mod byte_orders;

pub fn info(filename: &str) {
    let mut file_handle;
    match File::open(filename) {
        Ok(handle) => file_handle = handle ,
        Err(_e) => panic!("Handle error!"),
    }

    let header = header::Header::new(& mut file_handle);
    println!("Header: {}", header);
}
