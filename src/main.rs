/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   will (maybe?) implement using rust libc calls once I know it's
*   working with the abstraction crates
*/

mod sd_commands;
use rppal::spi::{Spi, Bus, Mode, SlaveSelect};
use crate::sd_commands::spi_cmd::{SpiCmd};


fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    //this is just an ioctl call, somehow calls to libc
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 
                           200_000, Mode::Mode0).unwrap();

    // for reading in responses from sd
    // response is within 1-8 bytes reply can be 1-2 bytes (so size 10)
    let mut in_buf: [u8; 10] = [0; 10];
    
    let cmd_0 = SpiCmd {
        index: 0x40,
        arg: [0; 4],
        crc: 0x95,
    };

    let cmd_1 = SpiCmd {
        index: 0x41,
        arg: [0; 4],
        crc: 0x00,
    };

    let cmd_8 = SpiCmd {
        index: 0x48,
        arg: [0x00, 0x00, 0x01, 0xaa],
        crc: 0x87,
    };

    let _test_cmd = SpiCmd {
        index: 0x81,
        arg: [0; 4],
        crc: 0x55,
    };

    // for i in cmd_0.buff() { print!("{}", i); }
    // print!("\n");

    // write 74+ bits with mosi and cs high.
    let out_buf: [u8; 10] = [0xff; 10];
    spi.write(&out_buf)?;

    // software reset: send sd cmd0 with chipselect low (w CRC)
    // card should go to idle state
    spi.write(&cmd_0.buff())?;

    spi.read(&mut in_buf)?;

    for i in in_buf { print!("{:x}, ", i); }
    print!("\n");

    // high capacity card:
    // send sd cmd8 (w CRC) with argument 0x1aa before init
    spi.write(&cmd_8.buff())?;

    spi.read(&mut in_buf)?;
    for i in in_buf { print!("{:x}, ", i); }
    print!("\n");

    // if 0x1aa matches: acmd41 arg: 0x40000000
    // if 0x1aa timeout: acmd41 arg: 0x00000000
    // if acmd41 timeout: cmd1
    // if cmd1 timeout: error

    // initialzation begins w sd cmd1, 
    
    return Ok(());
}
