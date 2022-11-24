/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   preliminary "proof of concept" stage, will (maybe?)
*   implement using rust libc calls once I know it's working
*   with the abstraction crates
*/

use rppal::spi::{Spi, Bus, Mode, SlaveSelect};
use gpio_cdev::{Chip, LineRequestFlags};
use std::thread::{sleep};

//spi command template
struct SpiCmd {
    index: u8,
    arg: [u8; 4],
}

impl SpiCmd {
    // hopefully calculates crc7
    fn crc7(&self) -> u8 {
        let mut sum:u64 = 0;
        sum &= (self.index as u64) << (5 * 8);
        sum &= (self.arg[0] as u64) << (4 * 8);
        sum &= (self.arg[1] as u64) << (3 * 8);
        sum &= (self.arg[2] as u64) << (2 * 8);
        sum &= (self.arg[3] as u64) << (1 * 8);
        sum = sum.pow(7) + sum.pow(3) + 1;
        return sum as u8;
    }

    // returns buffer to write entire command, including crc
    fn buff(&self) -> [u8; 6] {
        return [self.index, 
                self.arg[0], self.arg[1], self.arg[2], self.arg[3],
                self.crc7()]
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //this is just an ioctl call, somehow calls to libc
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 
                           200_000, Mode::Mode0).unwrap();
    // need a gpio pin, 
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let handle = chip
        .get_line(23)?
        .request(LineRequestFlags::OUTPUT, 1, "rust_gpio")?;

    // for reading in responses from sd
    // response is within 1-8 bytes, and i belive the reply can be 1-2 bytes
    // hence the 10 byte buffer. not sure if the read works like that.
    let mut in_buf: [u8; 10] = [0; 10];
    
    let cmd_0 = SpiCmd {
        index: 0x40,
        arg: [0; 4],
    };

    let _test_cmd = SpiCmd {
        index: 0x00,
        arg: [0; 4],
    };

    // for i in cmd_0.buff() { print!("{}", i); }
    // print!("\n");

    // TODO: supposed to toggle with chipselect high, .write() uses cs low
    // -> software: toggle gpio really fast?
    // -> hardware: break out the transistors and do an open drain kinda thing?
    // write 74+ bits with mosi and cs high.
    let out_buf: [u8; 80] = [0xff; 80];
    let mut rv = spi.write(&out_buf);
    println!("write: {:?}", rv);

    // software reset: send sd cmd0 with chipselect low
    // and correct CRC code
    // card should go to idle state

    handle.set_value(0);
    rv = spi.write(&cmd_0.buff());
    handle.set_value(1);

    println!("write: {:?}", rv);
    println!("{:#x}", &cmd_0.buff()[5]);

    handle.set_value(0);
    rv = spi.read(&mut in_buf);
    handle.set_value(1);
    println!("read: {:?}", rv);

    // high capacity card:
    // send sd cmd8 with argument 0x1aa before init
    // crc must be correct

    // if 0x1aa matches: acmd41 arg: 0x40000000
    // if 0x1aa timeout: acmd41 arg: 0x00000000
    // if acmd41 timeout: cmd1
    // if cmd1 timeout: error

    // initialzation begins w sd cmd1, 
    
    return Ok(());
}
