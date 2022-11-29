/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   main.rs
*/

mod sd_commands;
mod pwm;
use rppal::spi::{Spi, Bus, Mode, SlaveSelect};
use rppal::uart::{Uart, Parity};
//use crate::pwm::pwm_init::{pwm_init};
use crate::sd_commands::sd_init::{sd_init};
use crate::sd_commands::sd_write::*;
use crate::sd_commands::sd_read::{one_block_pretty_print,
                                  read_sd_1_block,
                                  sd_multiblock_read, 
                                  multiblock_pretty_print};

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    //this is essentially a wrapper for the ioctl call
    let spi = &mut Spi::new(Bus::Spi0, SlaveSelect::Ss0, 
                           125_000, Mode::Mode0).unwrap();

    // sends init sequence to sd card
    sd_init(spi)?;

    // reserved sectors in partition
    for i in 0..2 {
        println!("\nsector: {:x}", 0x0800 + i);
        sd_send_cmd(spi, CMD_17, 0x800+i)?;
        one_block_pretty_print(read_sd_1_block(spi)?);
    }

    // data start (fat table)
    let mut sector: u32 = 0x800 + 32 + 0x3b80*2; //2*FATSz sectors past reserved
    println!("\nsector: {:x}", sector);
    sd_send_cmd(spi, CMD_17, sector)?;
    one_block_pretty_print(read_sd_1_block(spi)?);

    println!("\npretty:\n");
    let marley_sector: u32 = sector + (0x0e-2)*32;
    multiblock_pretty_print(sd_multiblock_read(spi, marley_sector+20, 10)?);

    let mut uart = Uart::new(115_200, Parity::None, 8, 1).unwrap();
    sd_send_cmd(spi, CMD_17, sector)?;
    let data = read_sd_1_block(spi)?.1;
    uart.write(&data)?;
    
    return Ok(());
}
