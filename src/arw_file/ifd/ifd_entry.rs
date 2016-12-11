use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;
use std::collections::HashMap;
use std::fmt;
use arw_file::byte_order;
use arw_file::ifd::tag;

use arw_file::format;

#[derive(Clone)]
pub struct FieldType {
    name: String,
    width: u8,
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({}b)", self.name, self.width)
    }
}

lazy_static! {
    pub static ref IFDFieldTypes : HashMap<u16, FieldType> = {
        let mut m = HashMap::new();
        m.insert(1, FieldType {name: String::from("BYTE"), width: 1});
        m.insert(2, FieldType {name: String::from("ASCII"), width: 1});
        m.insert(3, FieldType {name: String::from("SHORT"), width: 2});
        m.insert(4, FieldType {name: String::from("LONG"), width: 4});
        m.insert(5, FieldType {name: String::from("RATIONAL"), width: 8});
        m.insert(6, FieldType {name: String::from("SBYTE"), width: 4});
        m.insert(7, FieldType {name: String::from("UNDEFINED"), width: 1});
        m.insert(8, FieldType {name: String::from("SSHORT"), width: 2});
        m.insert(9, FieldType {name: String::from("SLONG"), width: 4});
        m.insert(10, FieldType {name: String::from("SRATIONAL"), width: 8});
        m.insert(11, FieldType {name: String::from("FLOAT"), width: 4});
        m.insert(12, FieldType {name: String::from("DOUBLE"), width: 8});
        m
    };
}

fn u16_to_field_type(val: u16) -> FieldType {
    if IFDFieldTypes.contains_key(&val) {
        IFDFieldTypes[&val].clone()
    } else {
        FieldType {
            name: String::from("Unknown"),
            width: 1,
        }
    }
}

pub struct IFDEntry {
    pub tag: tag::Tag,
    pub field_type: FieldType,
    pub count: u32, // u32 number of values, count of the indicated type
    pub value_offset: u32, // u32 the value offset OR the value, if the type fits 4bytes :)
    pub value_bytes: Vec<u8>,
    pub byte_order: byte_order::ByteOrder,
}

impl IFDEntry {
    pub fn new(mut f: &mut File,
               offset: u32,
               byte_order: byte_order::ByteOrder,
               ifd_type: &String)
               -> IFDEntry {

        match (*f).seek(SeekFrom::Start(offset as u64)) {
            Ok(position) => {
                if position != (offset as u64) {
                    panic!("Error, can't seek to the ifd entry offset")
                }
            }
            Err(e) => panic!("Cannot read ifd entry: {}", e),
        }

        let mut buf = vec![0; 12];

        match (*f).read(&mut buf) {
            Ok(n) => {
                if n < 12 {
                    panic!("IFD entry incomplete")
                }
            }
            Err(e) => panic!("Error: {}", e),
        }
        let tag_id = byte_order.parse_u16(&buf[0..2]);

        let field_type = u16_to_field_type(byte_order.parse_u16(&buf[2..4]));
        let count = byte_order.parse_u32(&buf[4..8]);
        let value_offset = byte_order.parse_u32(&buf[8..12]);

        let byte_width = (count * field_type.width as u32) as usize;

        IFDEntry {
            value_bytes: IFDEntry::value_bytes(&mut f, byte_width, &byte_order, value_offset),
            tag: IFDEntry::tag_for_id(tag_id, *ifd_type == String::from("MakerNote")),
            field_type: field_type,
            count: count,
            value_offset: value_offset,
            byte_order: byte_order,
        }
    }

    fn tag_for_id(id: u16, sony_tags: bool) -> tag::Tag {
        let tag_result = if sony_tags {
            tag::SONY_TAGS.get(&id)
        } else {
            tag::TAGS.get(&id)
        };

        match tag_result {
            Some(tag) => (*tag).clone(),
            None => {
                tag::Tag {
                    id: 0,
                    ifd: false,
                    label: format!("Unknown tag {}", &id),
                    description: String::from(""),
                }
            }
        }

    }

    pub fn value_bytes(f: &mut File,
                       count: usize,
                       byte_order: &byte_order::ByteOrder,
                       value_offset: u32)
                       -> Vec<u8> {

        let mut buf = vec![0; count];
        if count <= 4 {
            // Fill buffer from value field
            return byte_order.u32_to_slice(value_offset).to_vec();
        } else {
            //
            match (*f).seek(SeekFrom::Start(value_offset as u64)) {
                Ok(pos) => {
                    if pos != (value_offset as u64) {
                        panic!("Error, can't seek to the field value offset");
                    }
                }
                Err(e) => panic!("Cannot read field value: {}", e),
            }

            match (*f).read(&mut buf) {
                Ok(n) => {
                    if n < count {
                        panic!("Field value incomplete");
                    }
                }
                Err(e) => panic!("Error: {}", e),
            }
            return buf;
        }
    }

    pub fn string_value(&self) -> String {
        match self.field_type.name.as_ref() {
            "ASCII" => self.ascii_value().unwrap(),
            "LONG" => format::vec_to_string(&self.long_values().unwrap()),
            "SHORT" => format::vec_to_string(&self.short_values().unwrap()),
            "SLONG" => format::vec_to_string(&self.signed_long_values().unwrap()),
            "SSHORT" => format::vec_to_string(&self.signed_short_values().unwrap()),
            _ => format::format_bytes(&self.value_bytes),
        }
    }

    pub fn ascii_value(&self) -> Option<String> {
        if self.field_type.name != String::from("ASCII") {
            return None;
        }
        match String::from_utf8(self.value_bytes.to_vec()) {
            Ok(str) => return Some(str),
            Err(_) => return None,
        }
    }

    pub fn long_values(&self) -> Option<Vec<u32>> {
        if self.field_type.name != String::from("LONG") {
            return None;
        }
        let iter = self.value_bytes.chunks(self.field_type.width as usize);
        let values: Vec<u32> = iter.map(|bytes_arr| self.byte_order.parse_u32(bytes_arr)).collect();
        return Some(values);
    }

    pub fn short_values(&self) -> Option<Vec<u16>> {
        if self.field_type.name != String::from("SHORT") {
            return None;
        }
        let iter = self.value_bytes.chunks(self.field_type.width as usize);
        let values: Vec<u16> = iter.map(|bytes_arr| self.byte_order.parse_u16(bytes_arr)).collect();
        return Some(values);
    }

    pub fn signed_long_values(&self) -> Option<Vec<i32>> {
        if self.field_type.name != String::from("SLONG") {
            return None;
        }
        let iter = self.value_bytes.chunks(self.field_type.width as usize);
        let values: Vec<i32> = iter.map(|bytes_arr| self.byte_order.parse_i32(bytes_arr)).collect();
        return Some(values);
    }

    pub fn signed_short_values(&self) -> Option<Vec<i16>> {
        if self.field_type.name != String::from("SSHORT") {
            return None;
        }
        let iter = self.value_bytes.chunks(self.field_type.width as usize);
        let values: Vec<i16> = iter.map(|bytes_arr| self.byte_order.parse_i16(bytes_arr)).collect();
        return Some(values);
    }

    pub fn is_ifd(&self) -> bool {
        self.tag.ifd || self.tag.label == String::from("MakerNote")
    }
}

impl fmt::Debug for IFDEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ascii_value = match self.ascii_value() {
            Some(str) => str,
            None => String::from("-"),
        };

        write!(f,
               "(ArwFile::IFDEntry tag: {}, field_type: {:?}, count: {}, value_offset: {}, \
                ascii_value: {})",
               self.tag,
               self.field_type,
               self.count,
               self.value_offset,
               ascii_value)
    }
}
