/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   will (maybe?) implement using rust libc calls once I know it's
*   working with the abstraction crates
*/

mod sd_commands;
use rppal::spi::{Spi, Bus, Mode, SlaveSelect};
use crate::sd_commands::sd_init::{sd_init};

fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    //this is just an ioctl call, somehow calls to libc
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 
                           125_000, Mode::Mode0).unwrap();

    sd_init(&mut spi)?;

    
    return Ok(());
}
