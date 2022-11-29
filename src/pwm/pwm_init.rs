/*  Michael Schmitt 2022
*   raspberry pi sd card spi implement
*
*   pwm_init.rs
*
*   initialize pwm io for use in pwm audio
*/

use rppal::pwm::{Pwm, Channel, Polarity};

pub fn pwm_init() -> Result<(), rppal::pwm::Error>
{
    // start a new pwm on channel 0 (gpio 18 / pin 12)
    let pwm = &mut Pwm::with_frequency(Channel::Pwm0,
                                           44_100_f64,
                                           0.5_f64,
                                           Polarity::Normal,
                                           false).unwrap();
    pwm.enable()?;
    loop {}
}
