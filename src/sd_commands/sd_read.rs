/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   sd_read.rs
*   
*   reads sd card responses 
*
*   response comes in various forms, doesn't come at exact
*   timing
*/

pub const R1_READ_SIZE: usize = 10;
pub const R3R7_READ_SIZE: usize = 16;
pub const ONE_BLOCK_READ_SIZE: usize = 900;
pub const BLOCK_SIZE: usize = 512;

use crate::sd_commands::sd_write::*;

// generate masks of different sizes
fn mask_zeros_from_msb_u8(num: u8) -> u8
{
    let base: u8 = 2;
    return base.pow((8 - num) as u32)-1;
}

fn mask_zeros_from_lsb_u8(num: u8) -> u8
{
    let base: u8 = 2;
    return 0xff-(base.pow(num as u32)-1);
}

// concatenate 4 u8 to a u32
// a in arguments stands for almost?
fn four_u8_to_u32(msb: u8, amsb: u8, alsb: u8, lsb: u8) -> u32
{
    let mut result: u32;
    result = (lsb as u32) << 0;
    result += (alsb  as u32) << 8;
    result += (amsb as u32) << 16;
    result += (msb as u32) << 24;

    return result;
}

pub fn read_sd_r1(spi: &mut rppal::spi::Spi) ->
    Result<u8, rppal::spi::Error>
{
    // response is within 1-8 bytes reply can be 1-2 bytes (so size 10)
    let mut buf: [u8; R1_READ_SIZE] = [0x00; R1_READ_SIZE];
    spi.transfer(&mut buf, &[0xff; R1_READ_SIZE])?;
    //spi.read(&mut buf)?;
    
    //for i in buf { print!("{:02x}, ", i); }
    //print!("\n");

    let mut response: u8;
    let mut k: u8;

    for i in 0..(R1_READ_SIZE - 1) {         //for each u8 in buf
        k = 0;
        while k < 8 {       //for each bit in u8
            // check for start of response
            if ((buf[i] << k) & 0x80) == 0 { 
                // if in a single u8
                if k == 0 {
                    return Ok(buf[i]);
                }
                // if split over two u8's
                response = (buf[i] & mask_zeros_from_msb_u8(k)) << k;
                response += (buf[i+1] & mask_zeros_from_lsb_u8(8-k)) >> (8-k);
                return Ok(response);
            }
            k += 1;         //next bit in u8
        }
    }
    return Ok(0xff);
}

pub fn read_sd_r3r7(spi: &mut rppal::spi::Spi) ->
    Result<(u8, u32), rppal::spi::Error>
{
    // response is within 1-8 bytes reply can be 1-2 bytes (so size 10)
    let mut buf: [u8; R3R7_READ_SIZE] = [0; R3R7_READ_SIZE];
    //spi.read(&mut buf)?;
    spi.transfer(&mut buf, &[0xff; R3R7_READ_SIZE])?;
    
    //for i in buf { print!("{:x}, ", i); }
    //print!("\n");

    let mut k: u8;

    for i in 0..(R3R7_READ_SIZE - 1) {         //for each u8 in buf
        k = 0;
        while k < 8 {       //for each bit in u8
            // check for start of response
            if ((buf[i] << k) & 0x80) == 0 { 

                // if in a single u8:
                if k == 0 {
                    return Ok((buf[i], four_u8_to_u32(buf[i+1],
                                                      buf[i+2],
                                                      buf[i+3],
                                                      buf[i+4])));
                }
                todo!("make a better r3/r7 read");

                // if split over two u8's:
                // assumes that response always fits in buffer
 
                //r1 = (buf[i] & mask_zeros_from_msb_u8(k)) << k;
                //r1 += (buf[i+1] & mask_zeros_from_lsb_u8(8-k)) >> (8-k);

                //return Ok((r1, 0));
            }
            k += 1;         //next bit in u8
        }
    }
    return Ok((0xff, 0xff));
}

// TODO: add structure DataPacket
// reads one block response from cmd17
pub fn read_sd_1_block(spi: &mut rppal::spi::Spi) ->
    Result<(u8, [u8; BLOCK_SIZE]), rppal::spi::Error>
    // result contains cmd response, data token, data block
{
    // response is within 1-8 bytes reply can be 1-2 bytes (so size 10)
    let mut buf: [u8; ONE_BLOCK_READ_SIZE] = [0x00; ONE_BLOCK_READ_SIZE];
    //spi.read(&mut buf)?;
    spi.transfer(&mut buf, &[0xff; ONE_BLOCK_READ_SIZE])?;
    
    //for i in buf { print!("{:x}, ", i); }
    //print!("\n");

    let mut r1: u8 = 0x00;
    let mut data_block: [u8; BLOCK_SIZE] = [0x00; BLOCK_SIZE];
    let mut k: u8;
    let mut r1_index: usize = 0;

    'outer: for i in 0..(ONE_BLOCK_READ_SIZE - 1) {         //for each u8 in buf
        k = 0;
        '_inner: while k < 8 {       //for each bit in u8

            // check for start of command response
            if ((buf[i] << k) & 0x80) == 0 { 

                // if starting in a single u8:
                if k == 0 {
                    r1 = buf[i];
                    r1_index = i;
                    //println!("cmd response: {:02x}, index: {}", r1, r1_index);
                    break 'outer;
                } else {
                    todo!("make a better block read");
                }
            }
            k+=1;
        }
    }

    // keep searching the response for the data block
    // looking for data token = 0xfe
    for i in (r1_index+1)..(ONE_BLOCK_READ_SIZE-1) {
        if buf[i] == 0xfe { 
            let mut m: usize = 0;
            for h in (i+1)..(i+BLOCK_SIZE+1) { //next 512 is data block
                data_block[m] = buf[h];
                m += 1;
            }
            return Ok((r1, data_block));
        }
    }

    return Ok((0xff, [0xff; BLOCK_SIZE]));
}

pub fn sd_multiblock_read(spi: &mut rppal::spi::Spi,
                          addr: u32,
                          block_count: usize) ->
    Result<(u8, u8, Vec<[u8; BLOCK_SIZE]>), rppal::spi::Error>
    //TODO: use block sized vector instead
{
    let r18: u8;
    let mut r12: u8 = 0xff;
    // send cmd18 to start read at addr
    sd_send_cmd(spi, CMD_18, addr)?;

    r18 = read_sd_r1(spi)?;

    // read each data packet into vector
    let mut vec_buf: Vec<[u8; BLOCK_SIZE]> 
        = vec![[0x00; BLOCK_SIZE]; block_count];


    let mut find_start_buf: [u8; 1] = [0xff];
    while find_start_buf[0] != 0xfe {
        spi.transfer(&mut find_start_buf, &[0xff; 1])?;
    }
    for i in 0..block_count {
        //spi.read(&mut buf)?;
        spi.transfer(&mut vec_buf[i], &[0xff; ONE_BLOCK_READ_SIZE])?;
    }

    // send cmd12 to stop the read
    sd_send_cmd_default(spi, CMD_12)?;

    // discard byte immediately after cmd12 and read the command response
    let mut buf: [u8; R1_READ_SIZE+1] = [0x00; R1_READ_SIZE+1];
    let mut k: u8;
    spi.transfer(&mut buf, &[0xff; R1_READ_SIZE+1])?;
    'outer: for i in 1..R1_READ_SIZE {
        k = 0;
        while k < 8 {
            if ((buf[i] << k) & 0x80) == 0 { 
                // if starting in a single u8:
                if k == 0 {
                    r12 = buf[i];
                    //println!("cmd response: {:02x}, index: {}", r1, r1_index);
                    break 'outer;
                } else {
                    todo!("make better multi-block read");
                }
            }
            k += 1;
        }
    }

    // wait until not busy (read 0xff, pulls low while busy)
    let mut busy_wait_buf: [u8; 1] = [0x00; 1];
    while busy_wait_buf[0] != 0xff {
        spi.transfer(&mut busy_wait_buf, &[0xff; 1])?;
    }

    return Ok((r18, r12, vec_buf));
}

pub fn one_block_pretty_print(block: (u8, [u8; BLOCK_SIZE]))
{
    println!("\ncmd r1: {:02x}", block.0);
    for i in (0..BLOCK_SIZE).step_by(16) {
        print!("{:03x}-{:03x} {: >3}-{: >3} {:02x?} {}\n", 
            i, i+15, i, i+15,                // print range
            &block.1[i..(i+16)],             //prints each hex value
            String::from_utf8_lossy(&block.1[i..(i+16)]));
    }
}
