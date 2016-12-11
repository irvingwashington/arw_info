#[derive(PartialEq, Clone, Copy)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

impl ByteOrder {
    pub fn to_str(&self) -> String {
        if *self == ByteOrder::BigEndian {
            String::from("BE")
        } else {
            String::from("LE")
        }
    }

    pub fn parse_u16(&self, buf: &[u8]) -> u16 {
        return if *self == ByteOrder::LittleEndian {
            ((buf[1] as u16) << 8) + (buf[0] as u16)
        } else {
            ((buf[0] as u16) << 8) + (buf[1] as u16)
        };
    }

    pub fn parse_u32(&self, buf: &[u8]) -> u32 {
        return if *self == ByteOrder::LittleEndian {
            ((buf[3] as u32) << 24) + ((buf[2] as u32) << 16) + ((buf[1] as u32) << 8) +
            (buf[0] as u32)
        } else {
            ((buf[0] as u32) << 24) + ((buf[1] as u32) << 16) + ((buf[2] as u32) << 8) +
            (buf[3] as u32)
        };
    }

    // Two's complement
    pub fn parse_i16(&self, buf: &[u8]) -> i16 {
        let mask: u16 = 32768; // 2:u16.pow(15)

        let input_value = self.parse_u16(&buf);
        (-((input_value & mask) as i32) + (input_value & !mask) as i32) as i16
    }

    // Two's complement
    pub fn parse_i32(&self, buf: &[u8]) -> i32 {
        let mask: u32 = 2147483648; // 2:u32.pow(31)

        let input_value = self.parse_u32(&buf);
        (-((input_value & mask) as i64) + (input_value & !mask) as i64) as i32
    }

    pub fn u32_to_slice(&self, val: u32) -> [u8; 4] {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        if *self == ByteOrder::LittleEndian {
            buf[3] = ((val >> 24) & 0xFF) as u8;
            buf[2] = ((val >> 16) & 0xFF) as u8;
            buf[1] = ((val >> 8) & 0xFF) as u8;
            buf[0] = (val & 0xFF) as u8;
        } else {
            buf[0] = ((val >> 24) & 0xFF) as u8;
            buf[1] = ((val >> 16) & 0xFF) as u8;
            buf[2] = ((val >> 8) & 0xFF) as u8;
            buf[3] = (val & 0xFF) as u8;
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_be_parse_u16() {
        let bo = ByteOrder::BigEndian;
        assert_eq!(bo.parse_u16(&[1, 0]), 0x100);
    }

    #[test]
    fn test_le_parse_u16() {
        let bo = ByteOrder::LittleEndian;
        assert_eq!(bo.parse_u16(&[0, 1]), 0x100);
    }

    #[test]
    fn test_be_parse_i16() {
        let bo = ByteOrder::BigEndian;
        assert_eq!(bo.parse_i16(&[255, 0]), -256);
    }

    #[test]
    fn test_le_parse_i16() {
        let bo = ByteOrder::LittleEndian;
        assert_eq!(bo.parse_i16(&[0, 255]), -256);
    }

    #[test]
    fn test_be_parse_u32() {
        let bo = ByteOrder::BigEndian;
        assert_eq!(bo.parse_u32(&[1, 0, 0, 0]), 0x1000000);
    }

    #[test]
    fn test_le_parse_u32() {
        let bo = ByteOrder::LittleEndian;
        assert_eq!(bo.parse_u32(&[0, 0, 0, 1]), 0x1000000);
    }

    #[test]
    fn test_be_parse_i32() {
        let bo = ByteOrder::BigEndian;
        assert_eq!(bo.parse_i32(&[255, 255, 255, 0]), -256);
    }

    #[test]
    fn test_le_parse_i32() {
        let bo = ByteOrder::LittleEndian;
        assert_eq!(bo.parse_i32(&[0, 255, 255, 255]), -256);
    }

    #[test]
    fn test_be_u32_to_slice() {
        let bo = ByteOrder::BigEndian;
        assert_eq!(bo.u32_to_slice(0x1000000), [1, 0, 0, 0]);
    }

    #[test]
    fn test_le_u32_to_slice() {
        let bo = ByteOrder::LittleEndian;
        assert_eq!(bo.u32_to_slice(0x1000000), [0, 0, 0, 1]);
    }
}
