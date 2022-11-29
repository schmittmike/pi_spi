/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   main.rs
*/

mod sd_commands;
use rppal::spi::{Spi, Bus, Mode, SlaveSelect};
use crate::sd_commands::sd_init::{sd_init};
use crate::sd_commands::sd_read::{one_block_pretty_print,
                                  read_sd_1_block,
                                  sd_multiblock_read};
use crate::sd_commands::sd_write::*;

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
    let mut sector: u32 = 0x800 + 32 + 0x3b80*2;
    println!("\nsector: {:x}", sector);
    sd_send_cmd(spi, CMD_17, sector)?;
    one_block_pretty_print(read_sd_1_block(spi)?);

    // data for test.txt start (4 sectors long)
    sector = 0x820 + 0x3b80*2 + (0x000d-0x2)*32;
    println!("\nsector: {:x}", sector);
    sd_send_cmd(spi, CMD_17, sector)?;
    one_block_pretty_print(read_sd_1_block(spi)?);

    sector = 0x820 + 0x3b80*2 + (0x000d-0x2)*32 + 1;
    println!("\nsector: {:x}", sector);
    sd_send_cmd(spi, CMD_17, sector)?;
    one_block_pretty_print(read_sd_1_block(spi)?);

    sector = 0x820 + 0x3b80*2 + (0x000d-0x2)*32 + 2;
    println!("\nsector: {:x}", sector);
    sd_send_cmd(spi, CMD_17, sector)?;
    one_block_pretty_print(read_sd_1_block(spi)?);

    sector = 0x820 + 0x3b80*2 + (0x000d-0x2)*32 + 3;
    println!("\nsector: {:x}", sector);
    sd_send_cmd(spi, CMD_17, sector)?;
    one_block_pretty_print(read_sd_1_block(spi)?);

    println!("\nmultiblock:\n");

    sector = 0x820 + 0x3b80*2 + (0x000d-0x2)*32;
    for i in 0..4 {
        println!("{:02x?}", sd_multiblock_read(spi, sector, 4)?.2[i]);
    }
    
    return Ok(());
}
