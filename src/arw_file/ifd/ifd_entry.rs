use std::mem::transmute;
use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;
use std::fmt;

use arw_file::byte_orders;
use arw_file::ifd::tag;

#[derive(Debug, PartialEq)]
pub enum IFDFieldType {
    BYTE = 1, // 8b
    ASCII, // 8b with 7bit ascii code, null terminated *
    SHORT, // 16b unsigned
    LONG, // 32b unsigned
    RATIONAL, // 64b - 32b unsigned numerator, 32b unsigned denominator *
    SBYTE, // 8b signed
    UNDEFINED, // 8b ?
    SSHORT, // 16b signed
    SLONG, // 32b signed
    SRATIONAL, // 64b - 32b signed numerator, 32b signed denominator *
    FLOAT, // 32b ieee float
    DOUBLE, // 64b ieee float  *
    UNKNOWN, // Internal, not from the specification *
}

impl IFDFieldType {
    fn from_u16(int_val: u16) -> IFDFieldType {
        if int_val > 0 && int_val < 13 {
            unsafe { transmute(int_val as i8) }
        } else {
            IFDFieldType::UNKNOWN
        }
    }
}

pub struct IFDEntry {
    pub tag: tag::Tag,
    pub field_type: IFDFieldType,
    pub count: u32, // u32 number of values, count of the indicated type
    value_offset: u32, // u32 the value offset OR the value, if the type fits 4bytes :)
    pub value_bytes: Vec<u8>,
}

impl IFDEntry {
    pub fn new(mut f: &mut File, offset: u32, byte_order: byte_orders::ByteOrders) -> IFDEntry {
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
        let tag = match tag::TAGS.get(&tag_id) {
            Some(tag) => (*tag).clone(),
            None => {
                tag::Tag {
                    id: 0,
                    label: format!("Unknown tag {}", &tag_id),
                    description: String::from("").clone(),
                }
            }
        };

        let field_type = byte_order.parse_u16(&buf[2..4]);
        let count = byte_order.parse_u32(&buf[4..8]);
        let value_offset = byte_order.parse_u32(&buf[8..12]);

        IFDEntry {
            value_bytes: IFDEntry::value_bytes(&mut f, count as usize, &byte_order, value_offset), // TODO: pass number of bytes in count
            tag: tag,
            field_type: IFDFieldType::from_u16(field_type),
            count: count,
            value_offset: value_offset,
        }
    }

    pub fn value_bytes(f: &mut File,
                       count: usize,
                       byte_order: &byte_orders::ByteOrders,
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

    pub fn ascii_value(&self) -> Option<String> {
        if self.field_type != IFDFieldType::ASCII {
            return None;
        }
        match String::from_utf8(self.value_bytes.to_vec()) {
            Ok(str) => return Some(str),
            Err(e) => return None,
        }
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
