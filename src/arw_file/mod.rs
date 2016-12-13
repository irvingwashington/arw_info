use std::fs::File;
mod header;
mod ifd;
mod byte_order;
mod format;

pub fn pretty_print(filename: &str, header: &header::Header) {
    println!("{} ({}), magic number: {}",
             filename,
             header.byte_order.to_str(),
             header.magic_number);
    println!("IFDs count: {}, first IFD offset: {}",
             header.ifds.len(),
             header.ifd_offset);

    for i in 0..header.ifds.len() {
        let ref ifd = header.ifds[i];
        println!("\nIFD {} ({}), entries: {}, offset: {}, next_offset: {} ",
                 i + 1,
                 ifd.ifd_type,
                 ifd.entries_count,
                 ifd.offset,
                 ifd.next_ifd_offset);
        for entry in &ifd.entries {
            println!("  {} ({:?}, {}): {}",
                     entry.tag,
                     entry.field_type,
                     entry.count,
                     entry.string_value());
        }
    }
}

pub fn info(filename: &str) {
    let mut file_handle;
    match File::open(filename) {
        Ok(handle) => file_handle = handle,
        Err(_e) => panic!("Handle error!"),
    }

    let header = header::Header::new(&mut file_handle);
    pretty_print(&filename, &header);
}
