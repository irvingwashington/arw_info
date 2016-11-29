use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;
use std::fmt;

use arw_file::byte_orders;

pub struct IFD {
    // Image File Directory
    entries_count: u16,
    entries: Vec<IFDEntry>, // 12b x n entries
    next_ifd_offset: u32, // u32 next ifd offset or 0
}

pub struct IFDEntry {
    tag: u16,
    field_type: u16,
    count: u32, // u32 number of values, count of the indicated type
    value_offset: u32, // u32 the value offset
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
            field_type: field_type,
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
               "(ArwFile::IFDEntry tag: {}, field_type: {}, count: {}, value_offset: {})",
               self.tag,
               self.field_type,
               self.count,
               self.value_offset)
    }
}

impl fmt::Debug for IFDEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "(ArwFile::IFDEntry tag: {}, field_type: {}, count: {}, value_offset: {})",
               self.tag,
               self.field_type,
               self.count,
               self.value_offset)
    }
}
