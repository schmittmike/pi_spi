//spi command template
pub struct SdCmd {
    pub index: u8,
    pub arg: [u8; 4],
    pub crc: u8,
}

impl SdCmd {
    // hopefully calculates crc7
    // TODO: it doesn't, found both required crc's on internet
    fn _crc7(&self) -> u8 {
        todo!()
    }

    // returns buffer to write entire command, including crc
    pub fn buff(&self) -> [u8; 6] {
        return [self.index, 
                self.arg[0], self.arg[1], self.arg[2], self.arg[3],
                self.crc]
    }

    pub fn with_arg(&self, new_arg: u32) -> [u8; 6] {
        return [self.index, 
                (new_arg & 0xff000000) as u8,
                (new_arg & 0x00ff0000) as u8,
                (new_arg & 0x0000ff00) as u8,
                (new_arg & 0x000000ff) as u8,
                self.crc]
    }
}
