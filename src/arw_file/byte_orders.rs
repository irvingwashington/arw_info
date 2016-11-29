#[derive(PartialEq)]
pub enum ByteOrders {
    LittleEndian,
    BigEndian,
}

impl ByteOrders {
    pub fn parse_u16(&self, buf: &[u8]) -> u16 {
        return if *self == ByteOrders::LittleEndian {
            ((buf[1] as u16) << 8) + (buf[0] as u16)
        } else {
            ((buf[0] as u16) << 8) + (buf[1] as u16)
        };
    }

    pub fn parse_u32(&self, buf: &[u8]) -> u32 {
        return if *self == ByteOrders::LittleEndian {
            ((buf[3] as u32) << 24) + ((buf[2] as u32) << 16) + ((buf[1] as u32) << 8) +
            (buf[0] as u32)
        } else {
            ((buf[0] as u32) << 24) + ((buf[1] as u32) << 16) + ((buf[2] as u32) << 8) +
            (buf[3] as u32)
        };
    }
}
