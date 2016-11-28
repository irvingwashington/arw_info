// TIFF is an image file format. In this document, a file is defined to be a sequence of 8-bit bytes,
// where the bytes are numbered from 0 to N. The largest possible TIFF file is 2**32 bytes in length.
// A TIFF file begins with an 8-byte image file header that points to an image file directory (IFD).
// An image file directory contains information about the image, as well as pointers to the actual image data.
// The following paragraphs describe the image file header and IFD in more detail.

use std::io;
use std::fs::File;
use std::io::Read;
use std::fmt;

#[derive(PartialEq)]
enum ByteOrders {
  LittleEndian,
  BigEndian
}

pub struct Header {
  byte_order: ByteOrders,
  // 0-1 The byte order used within the file. Legal values are:
  // II - little endian
  // MM - big endian

  magic_number: u16,
  // 2-3 An arbitrary but carefully chosen number (42) that further identifies the file as a TIFF file.

  ifd_offset: u32,
  // 4-7 the offset of the first IFD
}

impl Header {
  pub fn new(f: & mut File) -> Header {
    let mut buf = vec![0; 10];

    match (*f).read(&mut buf) {
      Ok(n) => { if n < 10 { panic!("Header incomplete"); } },
      Err(e) => panic!("Error {}", e),
    }

    let byte_order = if buf[0] == buf[1] && buf[1] == 73 {
      ByteOrders::LittleEndian
    } else if buf[0] == buf[1] && buf[1] == 77 {
      ByteOrders::BigEndian
    } else {
      panic!("Header byte order unknown!");
    };


    Header { byte_order: byte_order, magic_number: 42, ifd_offset: 11 }
  }
}

impl fmt::Display for Header {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let be_str = if self.byte_order == ByteOrders::LittleEndian {
      "LE"
    } else {
      "BE"
    };
    write!(f, "(ArwFile::Header byte_order: {}, magic number: {}, ifd_offset: {})", be_str, self.magic_number, self.ifd_offset)
  }
}