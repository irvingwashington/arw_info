use std::fs::File;
use std::io::Read;
use std::mem;
use arw_file::byte_order;
use arw_file::ifd;
use arw_file::ifd::IFDTuple;

const BE_MAGIC: u8 = 77;
const LE_MAGIC: u8 = 73;

pub struct Header {
    pub byte_order: byte_order::ByteOrder,
    pub magic_number: u16,
    pub ifd_offset: u32,
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
            byte_order::ByteOrder::LittleEndian
        } else if buf[0] == buf[1] && buf[1] == BE_MAGIC {
            byte_order::ByteOrder::BigEndian
        } else {
            panic!("Header byte order unknown!");
        };

        let magic_number = byte_order.parse_u16(&buf[2..4]);
        let ifd_offset = byte_order.parse_u32(&buf[4..8]);

        let mut offsets: Vec<IFDTuple> = vec![IFDTuple {
                                                  offset: ifd_offset,
                                                  tag_label: String::from("Main"),
                                              }];
        let mut ifds: Vec<ifd::IFD> = vec![];

        while !offsets.is_empty() {
            let taken_offsets: Vec<IFDTuple> = offsets.drain(0..).collect();

            for IFDTuple { offset, tag_label } in taken_offsets {
                let ifd = ifd::IFD::new(f, offset, &byte_order, &tag_label);

                for sub_ifd_tuple in ifd.sub_ifd_offsets() {
                    if sub_ifd_tuple.offset != 0 {
                        offsets.push(sub_ifd_tuple)
                    }
                }

                if ifd.next_ifd_offset != 0 {
                    offsets.push(IFDTuple {
                        offset: ifd.next_ifd_offset,
                        tag_label: tag_label,
                    })
                }
                ifds.push(ifd);
            }
        }

        Header {
            byte_order: byte_order,
            magic_number: magic_number,
            ifd_offset: ifd_offset,
            ifds: ifds,
        }
    }
}
