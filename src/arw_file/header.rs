use std::fs::File;
use std::io::Read;
use std::fmt;
use std::mem;
use arw_file::byte_orders;
use arw_file::ifd;

const BE_MAGIC: u8 = 77;
const LE_MAGIC: u8 = 73;

pub struct Header {
  byte_order: byte_orders::ByteOrders,
  // 0-1 The byte order used within the file. Legal values are:
  // II - little endian
  // MM - big endian

  magic_number: u16, // 2-3 An arbitrary but carefully chosen number (42) that further identifies the file as a TIFF file.
  ifd_offset: u32, // 4-7 the offset of the first IFD
  first_ifd: ifd::IFD,
}

impl Header {
  pub fn new(f: & mut File) -> Header {
    let buf_size = mem::size_of::<Header>();

    let mut buf = vec![0; buf_size];

    match (*f).read(&mut buf) {
      Ok(n) => { if n < buf_size { panic!("Header incomplete"); } },
      Err(e) => panic!("Error: {}", e),
    }

    let byte_order = if buf[0] == buf[1] && buf[1] == LE_MAGIC {
      byte_orders::ByteOrders::LittleEndian
    } else if buf[0] == buf[1] && buf[1] == BE_MAGIC {
      byte_orders::ByteOrders::BigEndian
    } else {
      panic!("Header byte order unknown!");
    };

    let magic_number = byte_order.parse_u16(& buf[2 .. 4]);
    let ifd_offset = byte_order.parse_u32(& buf[4 .. 8]);
    let first_ifd = ifd::IFD::new(f, ifd_offset, & byte_order);

    Header { byte_order: byte_order, magic_number: magic_number, ifd_offset: ifd_offset, first_ifd: first_ifd }
  }
}

impl fmt::Display for Header {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let be_str = if self.byte_order == byte_orders::ByteOrders::LittleEndian {
      "LE"
    } else {
      "BE"
    };
    write!(f, "(ArwFile::Header byte_order: {}, magic number: {}, ifd_offset: {}, first_ifd: {})",
      be_str, self.magic_number, self.ifd_offset, self.first_ifd)
  }
}
