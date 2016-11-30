use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;
use std::fmt;
use std::mem::transmute;

use arw_file::byte_orders;

#[derive(Debug, PartialEq)]
pub enum IFDFieldType {
    BYTE = 1, // 8b
    ASCII, // 8b with 7bit ascii code, null terminated
    SHORT, // 16b unsigned
    LONG, // 32b unsigned
    RATIONAL, // 64b - 32b unsigned numerator, 32b unsigned denominator
    SBYTE, // 8b signed
    UNDEFINED, // 8b ?
    SSHORT, // 16b signed
    SLONG, // 32b signed
    SRATIONAL, // 64b - 32b signed numerator, 32b signed denominator
    FLOAT, // 32b ieee float
    DOUBLE, // 64b ieee float
    UNKNOWN,
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

pub struct IFD {
    // Image File Directory
    pub entries_count: u16,
    pub entries: Vec<IFDEntry>, // 12b x entries_count entries
    pub next_ifd_offset: u32, // u32 next ifd offset or 0
}

pub struct IFDEntry {
    tag: u16,
    field_type: IFDFieldType,
    count: u32, // u32 number of values, count of the indicated type
    value_offset: u32, // u32 the value offset OR the value, if the type fits 4bytes :)
}

impl IFDEntry {
    pub fn new(f: &mut File, offset: u32, byte_order: &byte_orders::ByteOrders) -> IFDEntry {
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
        let tag = byte_order.parse_u16(&buf[0..2]);
        let field_type = byte_order.parse_u16(&buf[2..4]);
        let count = byte_order.parse_u32(&buf[4..8]);
        let value_offset = byte_order.parse_u32(&buf[8..12]);

        IFDEntry {
            tag: tag,
            field_type: IFDFieldType::from_u16(field_type),
            count: count,
            value_offset: value_offset,
        }
    }
}

impl IFD {
    pub fn new(mut f: &mut File, offset: u32, byte_order: &byte_orders::ByteOrders) -> IFD {
        let mut buf = vec![0; 4];

        match (*f).seek(SeekFrom::Start(offset as u64)) {
            Ok(position) => {
                if position != (offset as u64) {
                    panic!("Error, can't seek to the offset")
                }
            }
            Err(e) => panic!("Error: {}", e),
        }

        match (*f).read(&mut buf) {
            Ok(n) => {
                if n < 4 {
                    panic!("IFD incomplete");
                }
            }
            Err(e) => panic!("Error: {}", e),
        }

        let entries_count = byte_order.parse_u16(&buf[0..2]);

        let mut ifd_entry_offset = offset + 2;
        let mut entries = vec![];

        for _ in 0..entries_count {
            entries.push(IFDEntry::new(&mut f, ifd_entry_offset, &byte_order));
            ifd_entry_offset += 12;
        }

        match (*f).read(&mut buf) {
            Ok(n) => {
                if n < 4 {
                    panic!("IFD incomplete");
                }
            }
            Err(e) => panic!("Error: {}", e),
        }
        let next_ifd_offset = byte_order.parse_u32(&buf[0..4]);

        IFD {
            entries_count: entries_count,
            entries: entries,
            next_ifd_offset: next_ifd_offset,
        }
    }
}

impl fmt::Display for IFD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(ArwFile::IFD entries_count: {}, entries num: {}, next_ifd_offset: {}, entries: \
                {:?})",
               self.entries_count,
               self.entries.len(),
               self.next_ifd_offset,
               self.entries)
    }
}

impl fmt::Display for IFDEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(ArwFile::IFDEntry tag: {}, field_type: {:?}, count: {}, value_offset: {})",
               self.tag,
               self.field_type,
               self.count,
               self.value_offset)
    }
}

impl fmt::Debug for IFDEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(ArwFile::IFDEntry tag: {}, field_type: {:?}, count: {}, value_offset: {})",
               self.tag,
               self.field_type,
               self.count,
               self.value_offset)
    }
}
