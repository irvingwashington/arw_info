use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;
use std::fmt;

use arw_file::byte_orders;

pub struct IFD { // Image File Directory
  entries_count: u16,
  entries: Vec<IFDEntry>, // 12b x n entries
  next_ifd_offset: u32       // u32 next ifd offset or 0
}

pub struct IFDEntry {
  tag: u16,
  field_type: u16,
  count: u32,    // u32 number of values, count of the indicated type
  value_offset: u32 // u32 the value offset
}

impl IFD {
  pub fn new(f: & mut File, offset: u32, byte_order: & byte_orders::ByteOrders) -> IFD {
    let mut buf = vec![0; 12];

    match (*f).seek(SeekFrom::Start(offset as u64)) {
      Ok(position) => { if position != (offset as u64) { panic!("Error, can't seek to the offset") } },
      Err(e) => { panic!("Error: {}", e)}
    }

    match (*f).read(&mut buf) {
      Ok(n) => { if n < 12 { panic!("Header incomplete"); } },
      Err(e) => panic!("Error: {}", e),
    }

    let entries_count = byte_order.parse_u16(& buf[ 0 .. 2 ] );

    let ifd = IFD { entries_count: entries_count, entries: vec![], next_ifd_offset: 0 };
    return ifd;
  }
}

impl fmt::Display for IFD {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(ArwFile::IFD entries_count: {}, entries num: {}, next_ifd_offset: {})",
      self.entries_count, self.entries.len(), self.next_ifd_offset)
  }
}
