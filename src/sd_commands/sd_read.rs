/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   sd_read.rs
*   
*   reads sd card and finds response
*
*   response comes in various forms, doesn't come at exact
*   timing
*/

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
    Result<u8, Box<dyn std::error::Error>>
{
    // response is within 1-8 bytes reply can be 1-2 bytes (so size 10)
    let mut buf: [u8; 10] = [0x00; 10];
    spi.transfer(&mut buf, &[0xff; 10])?;
    //spi.read(&mut buf)?;
    
    //for i in buf { print!("{:02x}, ", i); }
    //print!("\n");

    let mut response: u8;
    let mut k: u8;

    for i in 0..9 {         //for each u8 in buf
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
    Result<(u8, u32), Box<dyn std::error::Error>>
{
    // response is within 1-8 bytes reply can be 1-2 bytes (so size 10)
    let mut buf: [u8; 16] = [0; 16];
    //spi.read(&mut buf)?;
    spi.transfer(&mut buf, &[0xff; 16])?;
    
    //for i in buf { print!("{:x}, ", i); }
    //print!("\n");

    let mut _r1: u8;
    let mut _r2: u32;
    let mut k: u8;

    for i in 0..15 {         //for each u8 in buf
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
