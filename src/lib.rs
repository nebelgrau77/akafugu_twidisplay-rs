//! A platform agnostic Rust driver for the Akafugu TWIDisplay 4-digit 7-segment display controller,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//! 
//! This driver allows you to:
//! 
//!## The device
//! The TWI 7-segment Display is an easy to use 4-digit 7-segment display that is controlled using the TWI (I2C compatible) protocol.
//! It is based on an ATMega4313 MCU, acting as a peripheral I2C device. 
//! 
//! ### Information: [TWIDisplay](https://www.akafugu.jp/posts/products/twidisplay/)
//! 
//! ## Usage examples (see also examples folder)
//!
//! Please find additional examples using hardware in this repository: [examples]
//!
//! [examples]: https://github.com/nebelgrau77/akafugu_twidisplay-rs/tree/main/examples
//! 
//! ### Initialization
//! 

// TO DO:
/*

- make the driver initialization with a specific address from an enum

Functions to add:

GENERAL:
- sending digits
- sending strings

COMMANDS:
DONE: - brightness setting, takes one argument 0-255 DONE
- address setting: takes one argument, actually can be anything between 0-127, so it can't be an enum, must be a number, with a match 
DONE: - clearing the display
- scroll/rotate mode: takes one argument, 0 or 1 (rotate/scroll) - this one can use an enum
- dots setting
- getting firmware revision
- getting number of digits
DONE: - display current I2C address

not tested by me yet: 
- custom character
- displaying a 16bit integer

- time setting - doesn't seem to be working

FUNCTIONS:
DONE - send digit
- send character
- send string
- display time (hh:mm:ss with a blinking dot)
DONE - display temperature (with a choice of C or F unit)


*/




#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]



use embedded_hal as hal;

use hal::blocking::i2c::{Write, WriteRead};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I2C bus error
    I2C(E),
    /// Invalid input data
    InvalidInputData,
}

struct Register;

// THESE WILL BE USED FOR VARIOUS OPERATIONS, E.G. SETTING POSITION
impl Register {
    // const CTRL_STATUS_1     : u8 = 0x00;
    const BRIGHTNESS_SETTING    :u8 = 0x80;
    const I2C_ADDRESS_SETTING   :u8 = 0x81;
    const CLEAR_DISPLAY         :u8 = 0x82;
    const MODE_SETTING          :u8 = 0x83;
    const CUSTOM_CHAR           :u8 = 0x84;
    const DOTS                  :u8 = 0x85;
    const TIME_SETTING          :u8 = 0x87; // not sure if this works
    const DISPLAY_WORD          :u8 = 0x88;
    const POSITION_SETTING      :u8 = 0x89;
    const FIRMWARE_REV          :u8 = 0x8a;
    const NUMBER_DIGITS         :u8 = 0x8b;
    const DISPLAY_ADDRESS       :u8 = 0x90;
}

struct BitFlags;

// THESE CAN BE USED FOR SETTING THE DOTS
impl BitFlags {
    //const TEST1                 : u8 = 0b1000_0000;
    
    }   

const DEVICE_ADDRESS: u8 = 0x12; // this is the main I2C address, but there can be two other addresses, so this should be an ~~enum~~ number instead


/// Two possible choices, used for various enable/disable bit flags
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]

// THIS DOESN'T SEEM NECESSARY
pub enum TempUnit {    
    /// Celsius degrees 
    C, 
    /// Fahrenheit degrees
    F,     
}



// THIS SHOULD CONTAIN THE ADDRESS AS WELL, LIKE IN THE LPS DRIVER
/// TWIDisplay driver
#[derive(Debug, Default)]
pub struct TWIDisplay<I2C> {
    /// The concrete I2C device implementation.
    i2c: I2C,
    dev_addr: u8,
}

// mod control;


impl <I2C, E> TWIDisplay<I2C>
where 
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance of the TWIDisplay driver.
    // NEEDS TO USE THE ADDRESS AS WELL
    pub fn new(i2c: I2C, dev_addr: u8) -> Self {
        TWIDisplay { i2c, dev_addr }
    }

    /// Destroy driver instance, return I2C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Clear the display
    pub fn clear_display(&mut self) -> Result<(), Error<E>> {
        self.i2c.write(self.dev_addr, &[Register::CLEAR_DISPLAY]).map_err(Error::I2C)
    }

    /// Display the current I2C address
    pub fn display_address(&mut self) -> Result<(), Error<E>> {
        self.i2c.write(self.dev_addr, &[Register::DISPLAY_ADDRESS]).map_err(Error::I2C)
    }

    /// Set brightness (0 - 255, 127 is 50%)
    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Error<E>> {
        self.i2c.write(self.dev_addr, &[Register::BRIGHTNESS_SETTING, brightness]).map_err(Error::I2C)
    }

    /// Write digit x at position y
    pub fn display_digit(&mut self, position: u8, digit: u8) -> Result<(), Error<E>> {
        self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, position, digit]).map_err(Error::I2C)
    }

    /// Display a number using all four digits
    pub fn display_number(&mut self, number: u16) -> Result<(), Error<E>> {
        let digits = TWIDisplay::<I2C>::get_digits(number);
        
        (0..4).for_each(|i| {
            self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING,
                                                 i as u8, 
                                                 digits[i as usize]])
                                                 .map_err(Error::I2C);    
        });     
   
        Ok(())
    }
    
    /// Display temperature with a unit and a minus sign if necessary
    pub fn display_temperature(&mut self, temperature: i8, unit: TempUnit) -> Result<(), Error<E>> {

        if temperature < (-99) {
            for (pos,ch) in "-LO-".chars().enumerate() {
                self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, pos as u8, ch as u8]).map_err(Error::I2C);
            }    
            Ok(())
        } else if temperature > 99 {
            for (pos,ch) in "-HI-".chars().enumerate() {
                self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, pos as u8, ch as u8]).map_err(Error::I2C);
            }    
            Ok(())
        } else {
            
        match temperature {
            t if t < 0 => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 0, '-' as u8]).map_err(Error::I2C),
            _ => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 0, ' ' as u8]).map_err(Error::I2C),
        };

        // insert a match here: if the decimal digits are zero, use a blank

        let decimals: u8 = (temperature.abs() / 10) as u8; 

        match decimals {
            0 => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 1, ' ' as u8]).map_err(Error::I2C),
            _ => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 1, decimals]).map_err(Error::I2C),
            }
        };

        //self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 1, (temperature.abs() / 10) as u8]).map_err(Error::I2C);
        self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 2, (temperature.abs()  % 10) as u8]).map_err(Error::I2C);

        match unit {
            TempUnit::F => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 3, 'F' as u8]).map_err(Error::I2C),
            TempUnit::C => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 3, 'C' as u8]).map_err(Error::I2C),
            //_ => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 3, ' ' as u8]).map_err(Error::I2C),
                
        };

        Ok(())

    }


    /// Get digits from a number
    fn get_digits(number: u16) -> [u8;4] {
        let mut data = number;
        let mut digits = [0u8;4];
        digits[0] = (data / 1000) as u8;
        data = data % 1000;
        digits[1] = (data / 100) as u8;
        data = data % 100;
        digits[2] = (data / 10) as u8;
        data = data % 10;
        digits[3] = data as u8;
        digits
    }


    // DO I REALLY NEED THESE FUNCTIONS? MAKES MORE SENSE TO GO DIRECTLY TO DOING STUFF...

    /*

    /// Write to a register.
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [register, data]; 
        self.i2c.write(DEVICE_ADDRESS, &payload).map_err(Error::I2C)
    }

    /// Read from a register. 
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

     */
}

