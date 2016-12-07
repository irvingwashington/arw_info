use std::fs::File;
use std::fmt::Write;
mod header;
mod ifd;
mod byte_orders;

pub fn pretty_print(filename: &str, header: &header::Header) {
    println!("{} ({}), magic number: {}",
             filename,
             header.byte_order.to_str(),
             header.magic_number);
    println!("IFDs count: {}", header.ifds.len());

    for i in 0..header.ifds.len() {
        let ref ifd = header.ifds[i];
        println!("\nIFD {} ({}), entries: {}, next_offset: {} ",
                 i + 1,
                 ifd.ifd_type,
                 ifd.entries_count,
                 ifd.next_ifd_offset);
        for entry in &ifd.entries {
            let entry_value: String;
            match entry.ascii_value() {
                Some(str) => entry_value = str,
                None => entry_value = format_bytes(&entry.value_bytes),
            }
            println!("  {} ({:?}, {}): {}",
                     entry.tag,
                     entry.field_type,
                     entry.count,
                     entry_value);
        }
    }
}

pub fn format_bytes(bytes: &Vec<u8>) -> String {
    let mut hex_form = String::new();

    for byte in (*bytes).iter().take(30) {
        write!(&mut hex_form, "{:02X} ", byte).unwrap();
    }
    if bytes.len() > 30 {
        write!(&mut hex_form, "(trunacted)").unwrap();
    }
    hex_form
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
