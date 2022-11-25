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

fn mask_zeros_from_msb(num: u8) -> u8
{
    let base: u8 = 2;
    return base.pow((8 - num) as u32)-1;
}

fn mask_zeros_from_lsb(num: u8) -> u8
{
    let base: u8 = 2;
    return 0xff-(base.pow(num as u32)-1);
}

pub fn read_sd_r1(spi: &mut rppal::spi::Spi) ->
    Result<u8, Box<dyn std::error::Error>>
{
    // response is within 1-8 bytes reply can be 1-2 bytes (so size 10)
    let mut buf: [u8; 10] = [0; 10];
    spi.read(&mut buf)?;
    
    //for i in buf { print!("{:x}, ", i); }
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
                response = (buf[i] & mask_zeros_from_msb(k)) << k;
                response += (buf[i+1] & mask_zeros_from_lsb(8-k)) >> (8-k);
                return Ok(response);
            }
            k += 1;         //next bit in u8
        }
    }
    return Ok(0xff);
}
