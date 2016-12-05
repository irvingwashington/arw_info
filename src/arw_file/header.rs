use std::fs::File;
use std::io::Read;
use std::fmt;
use std::mem;
use arw_file::byte_orders;
use arw_file::ifd;

const BE_MAGIC: u8 = 77;
const LE_MAGIC: u8 = 73;

pub struct Header {
    pub byte_order: byte_orders::ByteOrders,
    // 0-1 The byte order used within the file. Legal values are:
    // II - little endian
    // MM - big endian
    pub magic_number: u16,
    ifd_offset: u32, // 4-7 the offset of the first IFD
    pub ifds: Vec<ifd::IFD>,
}

impl Header {
    pub fn new(f: &mut File) -> Header {
        let buf_size = mem::size_of::<Header>();

        let mut buf = vec![0; buf_size];

        match (*f).read(&mut buf) {
            Ok(n) => {
                if n < buf_size {
                    panic!("Header incomplete");
                }
            }
            Err(e) => panic!("Error: {}", e),
        }

        let byte_order = if buf[0] == buf[1] && buf[1] == LE_MAGIC {
            byte_orders::ByteOrders::LittleEndian
        } else if buf[0] == buf[1] && buf[1] == BE_MAGIC {
            byte_orders::ByteOrders::BigEndian
        } else {
            panic!("Header byte order unknown!");
        };

        let magic_number = byte_order.parse_u16(&buf[2..4]);
        let ifd_offset = byte_order.parse_u32(&buf[4..8]);

        let mut next_ifd_offset = ifd_offset;
        let mut ifds: Vec<ifd::IFD> = vec![];

        while next_ifd_offset != 0 {
            let ifd = ifd::IFD::new(f, next_ifd_offset, &byte_order);
            next_ifd_offset = ifd.next_ifd_offset;
            ifds.push(ifd);
        }

        Header {
            byte_order: byte_order,
            magic_number: magic_number,
            ifd_offset: ifd_offset,
            ifds: ifds,
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let be_str = if self.byte_order == byte_orders::ByteOrders::LittleEndian {
            "LE"
        } else {
            "BE"
        };
        let mut res: fmt::Result;
        res = write!(f,
                     "(ArwFile::Header byte_order: {}, magic number: {}, ifd_offset: {})",
                     be_str,
                     self.magic_number,
                     self.ifd_offset);
        for ifd in &self.ifds {
            res = write!(f, "\n {}", ifd);
        }
        res
    }
}
