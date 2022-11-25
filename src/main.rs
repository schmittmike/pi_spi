/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   will (maybe?) implement using rust libc calls once I know it's
*   working with the abstraction crates
*/

mod sd_commands;
use rppal::spi::{Spi, Bus, Mode, SlaveSelect};
use crate::sd_commands::spi_cmd::{SpiCmd};
use crate::sd_commands::sd_read::{read_sd_r1};

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    //this is just an ioctl call, somehow calls to libc
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 
                           200_000, Mode::Mode0).unwrap();

    let cmd_0 = SpiCmd {
        index: 0x40,
        arg: [0; 4],
        crc: 0x95,
    };

    let _cmd_1 = SpiCmd {
        index: 0x41,
        arg: [0; 4],
        crc: 0x00,
    };

    let cmd_8 = SpiCmd {
        index: 0x48,
        arg: [0x00, 0x00, 0x01, 0xaa],
        crc: 0x87,
    };

    let cmd_55 = SpiCmd {
        index: 0x77,
        arg: [0; 4],
        crc: 0x65,
    };

    let cmd_58 = SpiCmd {
        index: 0x7a,
        arg: [0; 4],
        crc: 0x55,
    };

    let acmd_41_0 = SpiCmd {
        index: 0x69,
        arg: [0; 4],
        crc: 0xe5,
    };

    let acmd_41_4 = SpiCmd {
        index: 0x69,
        arg: [0x40, 0x00, 0x00, 0x00],
        crc: 0x77,
    };

    let _test_cmd = SpiCmd {
        index: 0xf1,
        arg: [0; 4],
        crc: 0x55,
    };

    // for i in cmd_0.buff() { print!("{}", i); }
    // print!("\n");

    // write 74+ bits (80) with mosi and cs high.
    let out_buf: [u8; 10] = [0xff; 10];     //0xff for mosi high
    spi.write(&out_buf)?;

    // software reset: send sd cmd0 with chipselect low (w CRC)
    // card should go to idle state
    spi.write(&cmd_0.buff())?;
    println!("cmd0 sd response: {:02x}", read_sd_r1(&mut spi)?);

    // high capacity card:
    // send sd cmd8 (w CRC) with argument 0x1aa before init
    spi.write(&cmd_8.buff())?;
    println!("cmd8 sd response: {:02x}", read_sd_r1(&mut spi)?);

    // if 0x1aa matches: acmd41 arg: 0x40000000
    spi.write(&cmd_55.buff())?;
    println!("cmd55 sd response: {:02x}", read_sd_r1(&mut spi)?);

    spi.write(&acmd_41_4.buff())?;
    println!("acmd41_0 sd response: {:02x}", read_sd_r1(&mut spi)?);

    // check CCS register bit 30 to see if it's sdhc
    spi.write(&cmd_58.buff())?;
    println!("cmd58 sd response: {:02x}", read_sd_r1(&mut spi)?);

    // if 0x1aa timeout: acmd41 arg: 0x00000000
    spi.write(&cmd_55.buff())?;
    println!("cmd55 sd response: {:02x}", read_sd_r1(&mut spi)?);

    spi.write(&acmd_41_0.buff())?;
    println!("acmd41_0 sd response: {:02x}", read_sd_r1(&mut spi)?);

    // if acmd41 timeout: cmd1
    // if cmd1 timeout: error

    // initialzation begins w sd cmd1, 
    
    return Ok(());
}
