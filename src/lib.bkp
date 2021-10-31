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

- time displaying - doesn't seem to be working

FUNCTIONS:
DONE - send digit
- send character
- send string
- display time (hh:mm:ss with a blinking dot)
DONE - display temperature (with a choice of C or F unit)


ADD READ FUNCTION
ADD DEFAULT ADDRESS AS A PUB CONST


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


/// Two possible choices for temperature units
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]

pub enum TempUnit {    
    /// Celsius degrees 
    C, 
    /// Fahrenheit degrees
    F,     
    // could add H for humidity
}
/// possible modes
pub enum Mode {
    /// Scroll
    Scroll,
    /// Rotate
    Rotate,
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

    /*
    /// Write to the I2C bus
    fn write(&mut self, payload: u8) -> Result<(), Error<E>> {
        self.i2c.write(self.dev_addr, &[payload]).map_err(Error::I2C)    
    }
     */


   /// Write to the I2C bus
   fn write(&mut self, payload: &[u8]) -> Result<(), Error<E>> {
    self.i2c.write(self.dev_addr, payload).map_err(Error::I2C)    
    }


    /// Clear the display
    pub fn clear_display(&mut self) -> Result<(), Error<E>> {
        self.write(&[Register::CLEAR_DISPLAY])?;
        Ok(())
        //self.i2c.write(self.dev_addr, &[Register::CLEAR_DISPLAY]).map_err(Error::I2C)
    }

    /// Display the current I2C address
    pub fn display_address(&mut self) -> Result<(), Error<E>> {
        self.write(&[Register::DISPLAY_ADDRESS])?;
        Ok(())
        //self.i2c.write(self.dev_addr, &[Register::DISPLAY_ADDRESS]).map_err(Error::I2C)
    }

    
    /// Set brightness (0 - 255, 127 is 50%)
    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Error<E>> {
        //self.i2c.write(self.dev_addr, &[Register::BRIGHTNESS_SETTING, brightness]).map_err(Error::I2C)
        //self.write(Register::BRIGHTNESS_SETTING);
        //self.write(brightness)
        self.write(&[Register::BRIGHTNESS_SETTING, brightness])?;
        Ok(())
        
    }

    /// Write digit x at position y
    pub fn display_digit(&mut self, position: u8, digit: u8) -> Result<(), Error<E>> {
        //self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, position, digit]).map_err(Error::I2C)
        //self.write(Register::POSITION_SETTING);
        //self.write(position);
        //self.write(digit)
        
        if position < 1 || 
           position > 4 ||
           digit > 9 {
            return Err(Error::InvalidInputData);
            } else {
                //self.write(&[Register::POSITION_SETTING, position, digit]).map_err(Error::I2C)
                self.write(&[Register::POSITION_SETTING, position, digit])?
            };
        
        Ok(())

    }

    // ADD A BOOLEAN SWITCH "with_leading_zeros"
    /// Display a number using all four digits
    pub fn display_number(&mut self, number: u16) -> Result<(), Error<E>> {
        
        if number > 9999 {
            return Err(Error::InvalidInputData);
        } 

        let digits = TWIDisplay::<I2C>::get_digits(number);
        
        for i in 0..4 {

            self.display_digit(i as u8, digits[i as usize])?
        }

        /*
        (0..4).for_each(|i| {
            
            self.display_digit(i as u8, digits[i as usize]).map_err(Error::I2C);
            //self.display_digit(i as u8, digits[i as usize]).map_err(Error::I2C);
                                        
        });
        */
        
        Ok(())

    }
    
    ///simply send a digit
    pub fn send_digit(&mut self, number: u8) -> Result<(), Error<E>> {
        
        if number > 9 {
            return Err(Error::InvalidInputData)
        } else {
            //self.i2c.write(self.dev_addr, &[number]).map_err(Error::I2C)
            //self.write(&[number]).map_err(Error::I2C)
            self.write(&[number])?
        };

        Ok(())

        

        //self.write(&[number]).map_err(Error::I2C)
        
        
    }
 

    /// set the mode
    pub fn set_mode(&mut self, mode: Mode) -> Result<(), Error<E>> {
        
        match mode {
            Mode::Rotate => self.write(&[Register::MODE_SETTING, 0])?,
            Mode::Scroll => self.write(&[Register::MODE_SETTING, 1])?,
        }
        
        Ok(())
    }

    // ADD A GENERIC display_value_with_unit_threshold(value, unit, lo, hi) FUNCTION
    // this can be used for temperature and humidity, and the simple version without threshold setting will just take lo = -99, hi = 99
    // function must check if lo < hi, otherwise defaults to -99,99
    // also if lo < -99 then lo = 99, if hi > 99 then hi = 99


    // CHANGE IT TO -99 / 999 AS DEFAULT
    //
    
    /*
        fn temp(t: i8, lo: Option<i8>, hi: Option<i8>) -> () {
        
            let mut lo_th = -99;
            let mut hi_th = 99;
            
            if let Some(l) = lo {
                lo_th = l
            }
            
            if let Some(h) = hi {
                hi_th = h
            }

            if lo_th < - 99 {
                lo_th = -99
            }
            
            if hi_th > 99 {
                hi_th = 99
            }


            if lo_th > hi_th {
                println!("temp: {}C, lo: {}, hi: {}", t, -99, 99)
            } else {
                println!("temp: {}C, lo: {}, hi: {}", t, lo_th, hi_th)
            }
            
        }


        fn temp_thresh(t: i8, lo: i8, hi: i8) -> () {
            temp(t, Some(lo), Some(hi))
        }
    
    
    */

    // NEED TO FIGURE OUT HOW TO SKIP LEADING ZEROS ALSO FOR 3-DIGIT NUMBERS

    // USE i16 INSTEAD OF i8

    /// Display temperature with a unit and a minus sign if necessary
    pub fn display_temperature(&mut self, temperature: i8, unit: TempUnit) -> Result<(), Error<E>> {

        if temperature < (-99) {
            for (pos,ch) in "-LO-".chars().enumerate() {
                //self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, pos as u8, ch as u8]).map_err(Error::I2C);
                self.write(&[Register::POSITION_SETTING, pos as u8, ch as u8])?;
            }    
            //Ok(())
        } else if temperature > 99 {
            for (pos,ch) in "-HI-".chars().enumerate() {
                //self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, pos as u8, ch as u8]).map_err(Error::I2C);
                self.write(&[Register::POSITION_SETTING, pos as u8, ch as u8])?;
            }    
            //Ok(())
        } else {
        
        // detect minus temperatures
        match temperature {
            //t if t < 0 => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 0, '-' as u8]).map_err(Error::I2C),
            //_ => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 0, ' ' as u8]).map_err(Error::I2C),
            t if t < 0 => self.write(&[Register::POSITION_SETTING, 0, '-' as u8])?,
            _ => self.write(&[Register::POSITION_SETTING, 0, ' ' as u8])?,
        };

        
        // add another match for hundreds, but only for positive numbers!
        // maybe the minus detection should happen here and if minus, then do just decimals matching, meanwhile for positive values also match hundreds

        // let hundreds: u8 = (temperature.abs() / 100) as u8;


        let decimals: u8 = (temperature.abs() / 10) as u8; 

        match decimals {
            //0 => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 1, ' ' as u8]).map_err(Error::I2C),
            //_ => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 1, decimals]).map_err(Error::I2C),
            0 => self.write(&[Register::POSITION_SETTING, 1, ' ' as u8])?,
            _ => self.write(&[Register::POSITION_SETTING, 1, decimals])?,
            }
            //Ok(())
        };

        //self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 1, (temperature.abs() / 10) as u8]).map_err(Error::I2C);
        //self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 2, (temperature.abs()  % 10) as u8]).map_err(Error::I2C);
        self.write(&[Register::POSITION_SETTING, 2, (temperature.abs()  % 10) as u8])?;
        

        match unit {
            //TempUnit::F => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 3, 'F' as u8]).map_err(Error::I2C),
            //TempUnit::C => self.i2c.write(self.dev_addr, &[Register::POSITION_SETTING, 3, 'C' as u8]).map_err(Error::I2C),
            TempUnit::F => self.write(&[Register::POSITION_SETTING, 3, 'F' as u8])?,
            TempUnit::C => self.write(&[Register::POSITION_SETTING, 3, 'C' as u8])?,
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


}

