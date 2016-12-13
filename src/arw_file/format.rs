use std::fmt::Write;
use std::fmt;

#[allow(unused_must_use)]
pub fn vec_to_string<T: fmt::Display>(collection: &Vec<T>) -> String {
    let mut str_form = String::new();

    if collection.len() > 1 {
        write!(&mut str_form, "[");
    };

    let mut first = true;

    for elem in collection {
        if !first {
            write!(&mut str_form, ", ");
        };
        write!(&mut str_form, "{}", elem);
        first = false;
    }
    if collection.len() > 1 {
        write!(&mut str_form, "]");
    };
    str_form
}

pub fn format_bytes(bytes: &Vec<u8>) -> String {
    let mut hex_form = String::new();

    for byte in (*bytes).iter().take(20) {
        write!(&mut hex_form, "{:02X} ", byte).unwrap();
    }
    if bytes.len() > 30 {
        write!(&mut hex_form, "(trunacted)").unwrap();
    }
    hex_form
}
