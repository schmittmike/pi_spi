//spi command template
pub struct SpiCmd {
    pub index: u8,
    pub arg: [u8; 4],
    pub crc: u8,
}

impl SpiCmd {
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
}
