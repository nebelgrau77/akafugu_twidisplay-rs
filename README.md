# Rust Akafugu TWIDisplay driver

![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

A platform agnostic Rust driver for the [Akafugu TWIDisplay](https://www.akafugu.jp/posts/products/twidisplay/)
based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Display single digits or characters, also at a selected position
- Display text, although some characters may not be available (see display documentation)
- Clear the display
- Show the current I2C address
- Change the I2C address (experimental function)
- Display time in HH.MM format
- Display temperature or humidity, with settable lower/upper threshold

### TO DO:
- [ ] test with other MCUs / Raspberry Pi
- [ ] add a good example
- [ ] read firmware version and number of digits (diagnostic functions)
- [ ] display dots at selected positions
  

## The device

The TWI 7-segment Display is an easy to use 4-digit 7-segment display that is controlled using the TWI (I2C compatible) protocol.
It is based on an ATMega4313 MCU, acting as a peripheral I2C device. 


## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [examples]

[examples]: https://github.com/nebelgrau77/akafugu_twidisplay-rs/tree/main/examples

```rust
#![no_main]
#![no_std]

use akafugu_twidisplay::*:



#[entry]
fn main() -> ! {

  // initialize all the necessary peripherals
  // create an instance of I2C bus

  let mut akafugu = TWIDisplay::new(i2c, DEFAULT_ADDRESS);
  
  akafugu.clear_display().unwrap();
  akafugu.set_brightness(127).unwrap();

  loop {

    let (hours, minutes, seconds) = some_rtc_reading();

    // display time in HH.MM format, dot is on when number of seconds is even

    if seconds % 2 == 0 {
      akafugu.display_time(hours, minutes, true).unwrap();
    } else {
      akafugu.display_time(hours, minutes, false).unwrap();
    }

  }

}

```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/nebelgrau77/akafugu_twidisplay-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

