/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   sd_read.rs
*   
*   wrapper for sending sd card commands 
*/

use crate::sd_commands::sd_cmd::{SdCmd};

pub const CMD_0: SdCmd = SdCmd {
    index: 0x40,
    arg: [0; 4],
    crc: 0x95,
};

pub const _CMD_1: SdCmd = SdCmd {
    index: 0x41,
    arg: [0; 4],
    crc: 0x00,
};

pub const CMD_8: SdCmd = SdCmd {
    index: 0x48,
    arg: [0x00, 0x00, 0x01, 0xaa],
    crc: 0x87,
};

pub const CMD_55: SdCmd = SdCmd {
    index: 0x77,
    arg: [0; 4],
    crc: 0x65,
};

pub const CMD_58: SdCmd = SdCmd {
    index: 0x7a,
    arg: [0; 4],
    crc: 0x55,
};

pub const _ACMD_41_0: SdCmd = SdCmd {
    index: 0x69,
    arg: [0; 4],
    crc: 0xe5,
};

pub const ACMD_41_4: SdCmd = SdCmd {
    index: 0x69,
    arg: [0x40, 0x00, 0x00, 0x00],
    crc: 0x77,
};

pub const _TEST_CMD: SdCmd = SdCmd {
    index: 0xf1,
    arg: [0; 4],
    crc: 0x55,
};

pub const CMD_17: SdCmd = SdCmd {
    index: 0x51,
    arg: [0x00; 4],
    crc: 0x55,
};

pub const CMD_18: SdCmd = SdCmd {
    index: 0x52,
    arg: [0x00; 4],
    crc: 0x55,
};

pub const CMD_12: SdCmd = SdCmd {
    index: 0x4c,
    arg: [0x00; 4],
    crc: 0x55,
};

// this send_cmd is for sending with "default" arg
pub fn sd_send_cmd_default(spi: &mut rppal::spi::Spi, 
                   cmd: SdCmd) -> Result<usize, rppal::spi::Error>
{
    return spi.write(&cmd.buff());
}

// this send_cmd is for adding your own argument
pub fn sd_send_cmd(spi: &mut rppal::spi::Spi, 
                   cmd: SdCmd, 
                   arg: u32) -> Result<usize, rppal::spi::Error>
{
    return spi.write(&cmd.with_arg(arg));
}

