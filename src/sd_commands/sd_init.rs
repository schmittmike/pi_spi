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

use crate::sd_commands::sd_cmd::{SdCmd};
use crate::sd_commands::sd_read::{read_sd_r1, read_sd_r3r7};

pub fn sd_init(spi: &mut rppal::spi::Spi) -> 
    Result<(), Box<dyn std::error::Error>>
{

    let cmd_0 = SdCmd {
        index: 0x40,
        arg: [0; 4],
        crc: 0x95,
    };

    let _cmd_1 = SdCmd {
        index: 0x41,
        arg: [0; 4],
        crc: 0x00,
    };

    let cmd_8 = SdCmd {
        index: 0x48,
        arg: [0x00, 0x00, 0x01, 0xaa],
        crc: 0x87,
    };

    let cmd_55 = SdCmd {
        index: 0x77,
        arg: [0; 4],
        crc: 0x65,
    };

    let cmd_58 = SdCmd {
        index: 0x7a,
        arg: [0; 4],
        crc: 0x55,
    };

    let _acmd_41_0 = SdCmd {
        index: 0x69,
        arg: [0; 4],
        crc: 0xe5,
    };

    let acmd_41_4 = SdCmd {
        index: 0x69,
        arg: [0x40, 0x00, 0x00, 0x00],
        crc: 0x77,
    };

    let _test_cmd = SdCmd {
        index: 0xf1,
        arg: [0; 4],
        crc: 0x55,
    };

    // write 74+ bits (80) with mosi and cs high.
    let out_buf: [u8; 10] = [0xff; 10];     //0xff for mosi high
    spi.write(&out_buf)?;

    // software reset: send sd cmd0 with chipselect low (w CRC)
    // card should go to idle state (r1: 0x01)
    spi.write(&cmd_0.buff())?;
    println!("cmd0 sd response: {:02x}", read_sd_r1(spi)?);

    // high capacity cards (most modern cards):
    // send sd cmd8 (w CRC) with argument 0x1aa before init
    // checks operating voltage, seems arbitrary but sometimes required
    // on success: "r7" (r1 with a 32 bit data field afterwards)
    spi.write(&cmd_8.buff())?;
    let rv: (u8, u32) = read_sd_r3r7(spi)?;
    println!("cmd8 sd r1: {:02x}\nr3/7: {:04x}", rv.0, rv.1);
    if rv.1 == 0x1AA {

        // if 0x1aa matches: acmd41 arg: 0x40000000 until ready
        spi.write(&cmd_55.buff())?;
        println!("cmd55 sd response: {:02x}", read_sd_r1(spi)?);
        spi.write(&acmd_41_4.buff())?;
        let mut k = read_sd_r1(spi)?;
        println!("acmd41_4: {:x}", k);

        while k != 0x00 {
            spi.write(&cmd_55.buff())?;
            println!("cmd55: {:02x}", read_sd_r1(spi)?);

            spi.write(&acmd_41_4.buff())?;
            k = read_sd_r1(spi)?;
            println!("acmd41_4: {:02x}", k);
        }

        // check CCS register bit 30 to see if it's sdhc (bit 30 = 1)
        spi.write(&cmd_58.buff())?;
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
    spi.set_clock_speed(2_000_000)?;

    Ok(())
}
