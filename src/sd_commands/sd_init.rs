/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   sd_init.rs
*   
*   initializes sd card
*
*   doesn't adhere to strict sd/mmc checking. works for my specific sd card,
*   could extend to full support but most cards nowadays are ver.2+ anyways
*/

use crate::sd_commands::sd_read::{read_sd_r1, read_sd_r3r7};
use crate::sd_commands::sd_write::*;

pub fn sd_init(spi: &mut rppal::spi::Spi) -> 
    Result<(), Box<dyn std::error::Error>>
{


    // write 74+ bits (80) with mosi and cs high.
    let out_buf: [u8; 10] = [0xff; 10];     //0xff for mosi high
    spi.write(&out_buf)?;

    // software reset: send sd cmd0 with chipselect low (w CRC)
    // card should go to idle state (r1: 0x01)
    sd_send_cmd_default(spi, CMD_0)?;
    println!("cmd0 sd response: {:02x}", read_sd_r1(spi)?);

    // high capacity cards (most modern cards):
    // send sd cmd8 (w CRC) with argument 0x1aa before init
    // checks operating voltage, seems arbitrary but sometimes required
    // on success: "r7" (r1 with a 32 bit data field afterwards)
    sd_send_cmd_default(spi, CMD_8)?;
    let rv: (u8, u32) = read_sd_r3r7(spi)?;
    println!("cmd8 sd r1: {:02x}\nr3/7: {:04x}", rv.0, rv.1);

    if rv.1 == 0x1AA {
        // if 0x1aa matches: acmd41 arg: 0x40000000 until ready
        sd_send_cmd_default(spi, CMD_55)?;
        println!("cmd55 sd response: {:02x}", read_sd_r1(spi)?);
        sd_send_cmd_default(spi, ACMD_41_4)?;
        let mut k = read_sd_r1(spi)?;
        println!("acmd41_4: {:x}", k);

        while k != 0x00 {
            sd_send_cmd_default(spi, CMD_55)?;
            println!("cmd55 sd response: {:02x}", read_sd_r1(spi)?);
            sd_send_cmd_default(spi, ACMD_41_4)?;
            k = read_sd_r1(spi)?;
            println!("acmd41_4: {:x}", k);
        }

        // check CCS register bit 30 to see if it's sdhc (bit 30 = 1)
        sd_send_cmd_default(spi, CMD_58)?;
        let rv: (u8, u32) = read_sd_r3r7(spi)?;
        println!("cmd58 sd r1: {:02x}\nr3/7: {:04x}", rv.0, rv.1);
        if rv.1 & 0x40000000 == 1 {
            println!("init success");
        }
    }

    // if 0x1aa timeout: acmd41 arg: 0x00000000

    // if acmd41 timeout: cmd1
    // if cmd1 timeout: error

    // initialzation begins w sd cmd1, 

    // faster clock allowed after init
    // this sometimes causes problems if the card doesn't respond in time.
    spi.set_clock_speed(5_000_000)?;

    Ok(())
}
