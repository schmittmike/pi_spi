/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   main.rs
*/

mod sd_commands;
use rppal::spi::{Spi, Bus, Mode, SlaveSelect};
use crate::sd_commands::sd_init::{sd_init};
use crate::sd_commands::sd_cmd::{SdCmd};
use crate::sd_commands::sd_read::{one_block_pretty_print, read_sd_1_block};

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    //this is just an ioctl call, somehow calls to libc
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 
                           125_000, Mode::Mode0).unwrap();

    sd_init(&mut spi)?;

    // faster clock allowed after init
    spi.set_clock_speed(3_000_000)?;

    let cmd_17 = SdCmd {
        index: 0x51,
        arg: [0x00; 4],
        crc: 0x55,
    };

    // reserved sectors in partition
    for i in 0..2 {
        println!("\nsector: {:x}", 0x0800 + i);
        spi.write(&cmd_17.with_arg(0x0800 + i))?;
        one_block_pretty_print(read_sd_1_block(&mut spi)?);
    }

    let sector: u32 = 0x820;
    println!("\nsector: {:x}", sector);
    spi.write(&cmd_17.with_arg(sector))?;
    one_block_pretty_print(read_sd_1_block(&mut spi)?);
    
    return Ok(());
}
