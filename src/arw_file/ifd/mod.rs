
use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;
use std::fmt;

use arw_file::byte_orders;

mod ifd_entry;
mod tag;

pub use self::ifd_entry::IFDEntry;

pub struct IFD {
    // Image File Directory
    pub entries_count: u16,
    pub entries: Vec<IFDEntry>, // 12b x entries_count entries
    pub next_ifd_offset: u32, // u32 next ifd offset or 0
    pub ifd_type: String,
}

impl IFD {
    pub fn new(mut f: &mut File,
               offset: u32,
               byte_order: &byte_orders::ByteOrder,
               ifd_type: &String)
               -> IFD {
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
            entries.push(IFDEntry::new(&mut f, ifd_entry_offset, *byte_order, &ifd_type));
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
            ifd_type: ifd_type.clone(),
        }
    }

    pub fn sub_ifds(&self, mut f: &mut File, byte_order: &byte_orders::ByteOrder) -> Vec<IFD> {
        let mut sub_ifds: Vec<IFD> = vec![];

        for entry in &self.entries {
            if entry.is_ifd() {
                let ifd = IFD::new(f, entry.value_offset, byte_order, &entry.tag.label);

                for sub_ifd in ifd.sub_ifds(f, byte_order) {
                    sub_ifds.push(sub_ifd);
                }

                sub_ifds.push(ifd);
            }
        } // TODO: Follow-up with next_ifd_offset

        sub_ifds
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
