/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   main.rs
*/

mod sd_commands;
mod pwm;
use std::{thread, time};
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
    let sector: u32 = 0x800 + 32 + 0x3b80*2; //2*FATSz sectors past reserved
    println!("\nsector: {:x}", sector);
    sd_send_cmd(spi, CMD_17, sector)?;
    one_block_pretty_print(read_sd_1_block(spi)?);

    println!("\npretty:\n");
    // wav data start
    let marley_sector: u32 = sector + (0x0e-2)*32;
    let bird_sector: u32 = sector + (0x0571-2)*32;
    sd_send_cmd(spi, CMD_17, marley_sector)?;
    one_block_pretty_print(read_sd_1_block(spi)?);

    multiblock_pretty_print(sd_multiblock_read(spi, marley_sector+25, 10)?);
    multiblock_pretty_print(sd_multiblock_read(spi, bird_sector, 10)?);

    let mut uart = Uart::new(115_200, Parity::None, 8, 1).unwrap();
    let mut some: u32 = 0;
    let mut buf_to_dac: [u8; 512] = [0x00; 512];
    let read_len: usize = 2;
    let mut sample: i16;
    const SAMPLE_SIZE: usize = 2;
    let mut sample_to_dac: [u8; SAMPLE_SIZE] = [0xff; SAMPLE_SIZE];

    loop {
        for i in (0..256).step_by(SAMPLE_SIZE) {
            for j in 0..SAMPLE_SIZE {
                sample_to_dac[j] = (j + i) as u8;
            }
            uart.write(&sample_to_dac);

            // (samples / sample rate) - 1
            thread::sleep(time::Duration::from_micros(44));
        }
    }
    
    //loop {
    //    let data = sd_multiblock_read(spi, marley_sector+some, read_len)?.4;
    //    for i in 0..2 {
    //        for j in 0..256 {
    //            println!("{:02x}, {:02x}", data[i][j*2], data[i][j*2+1]);
    //            sample = ((data[i][j*2+1] as i16) <<8)+((data[i][j*2]) as i16);
    //            println!("{:04x}", sample);
    //            sample_to_dac = (((sample as i32) + (1<<15)) >> 8) as u8;
    //            println!("{:02x}", sample_to_dac);
    //            buf_to_dac[j + i*j] = sample_to_dac;
    //        }
    //    }
    //    uart.write(&buf_to_dac)?;
    //    some += 2;
    //}
    
    //return Ok(());
}
