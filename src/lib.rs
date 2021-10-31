//! A platform agnostic Rust driver for the Akafugu TWIDisplay 4-digit 7-segment display controller,
//! based on the [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//! 
//! This driver allows you to:
//! - Display single digits or characters, also at a selected position
//! - Display text, although some characters may not be available (see display documentation)
//! - Clear the display
//! - Show the current I2C address
//! - Change the I2C address (experimental function)
//! - Display time in HH.MM format
//! - Display temperature or humidity, with settable lower/upper threshold
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
//! A new instance of the device is created as follows:
//! 
//! ```rust
//! use akafugu_twidisplay::*:
//! 
//! let mut akafugu = TWIDisplay::new(i2c, DEFAULT_ADDRESS);
//! ```
//! 
//! The default address is 0x12. If the address was changed with the `set_address()` function,
//! the new address must be used after a power down-power up sequence.  
//! 
//! 
//! ### Main functions
//! 
//! Display can be cleared with the following command:
//! ```rust
//! akafugu.clear_display().unwrap();
//! ```
//! 
//! Digits and/or characters can either be simply sent to display, or displayed at defined positions.
//! 
//! ```rust
//! // display digit '7' at position 2 (positions are 0,1,2,3 from left to right)
//! akafugu.display_digit(2, 7).unwrap();
//! // display character 'P' at position 3
//! akafugu.display_char(3,'P').unwrap();
//! ```
//! 
//! If a digit/character is just sent to the display, it will appear according to the selected mode
//! (scroll or rotate) - please see the documentation. 
//! 
//! ```rust
//! akafugu.send_char('A').unwrap();
//! akafugu.send_char('B').unwrap();
//! akafugu.send_char('C').unwrap();
//! akafugu.send_char('D').unwrap();
//! ```
//! 
//! This will display `ABCD`. 
//! 
//! ```rust
//! akafugu.send_char('E').unwrap();
//! ```
//! 
//! Depending on the selected mode the display will show now:
//! * in SCROLL mode: 'BCDE' 
//! * in ROTATE mode: 'EBCD'
//! 
//! Text can be sent to display as string literals:
//! 
//! ```rust
//! akafugu.send_text("HELLO LOOP PULL CALL").unwrap();
//! ```
//! 
//! Numbers from 0-9999 range can be displayed with the following function:
//! ```rust
//! akafugu.display_number(1234).unwrap();
//! ```
//! _NOTE_: Numbers will be displayed with leading zeroes, e.g. `0023`. 
//! 
//! ### Control functions
//! 
//! Display mode can be changed as follows:
//! 
//! ```rust
//! akafugu.set_mode(Mode::Scroll).unwrap(); // default mode is `Rotate`
//! ```
//! 
//! Brightness can be set between 0 and 255, where 127 is approx. 50% brightness.
//! ```rust 
//! akafugu.set_brightness(200).unwrap();
//! ```
//! 
//! The I2C address of the device can be changed from the default 0x12 as follows:
//! ```rust
//! akafugu.set_address(0x20).unwrap();
//! ```
//! 
//! The new address will be active after a power down, power up sequence.
//! 
//! __NOTE:__ According to the documentation, the allowed range of addresses is 0x00-0x7F,
//! but addresses including and over 0x40 don't seem to work correctly, even though
//! they are correctly displayed. In such case 0x00 must be used to access the device and change the address again. 
//! For this reason in this driver the address setting is restricted to 0x00-0x39 range.
//! 
//! To show the current I2C address use the following command:
//! ```rust
//! akafugu.display_address().unwrap();
//! ```
//! The same can be achieved by simply connecting only the VCC and GND pins of the display.
//! 
//! ### Convenience functions
//! The driver has three additional functions, that can be useful for clock or sensor applications.
//! 
//! #### Display time
//! Time is displayed in HH.MM format, with the central dot displayed or not:
//! 
//! ```rust
//! 
//! // get time from the clock 
//! let (hours, minutes, seconds) = some_rtc_function();
//! 
//! // blink the dot: on if number of seconds is even, otherwise off
//! if seconds % 2 == 0 {
//!     akafugu.display_time(hours, minutes, true).unwrap()
//! } else {
//!     akafugu.display_time(hours, minutes, false).unwrap()
//! }
//! ```
//! 
//! #### Display temperature
//! 


// TO DO:
/*



COMMANDS:
- dots setting
- getting firmware revision
- getting number of digits

not tested by me yet: 
- custom character
- displaying a 16bit integer

- time displaying - doesn't seem to be working



TO DO:

FUNCTIONS:

- READ() FUNCTION 
- READING FIRMWARE / VERSION
- SETTING I2C ADDRESS
- SETTING DOTS WITH BOOLEANS

USE '-' FOR DIGITS IF INCORRECT (DOES IT MAKE SENSE?)

OTHER:
- CUSTOM CHARACTERS


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
    const DOT2                    : u8 = 0b0000_0100;
    }   

/// Default I2C address for the device
pub const DEFAULT_ADDRESS: u8 = 0x12; 

/// Possible choices for temperature units
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]

pub enum TempUnits {    
    /// Celsius degrees 
    Celsius, 
    /// Fahrenheit degrees
    Fahrenheit,         
}

/// Two possible display modes
pub enum Mode {
    /// Scroll
    Scroll,
    /// Rotate
    Rotate,
}


/// TWIDisplay driver, that holds the I2C bus instance and the I2C address used
#[derive(Debug, Default)]
pub struct TWIDisplay<I2C> {
    /// The concrete I2C device implementation.
    i2c: I2C,
    dev_addr: u8,
}


impl <I2C, E> TWIDisplay<I2C>
where 
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    /// Create a new instance of the TWIDisplay driver.    
    pub fn new(i2c: I2C, dev_addr: u8) -> Self {
        TWIDisplay { i2c, dev_addr }
    }

    /// Destroy driver instance, return I2C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Write data to the I2C bus
    fn write(&mut self, payload: &[u8]) -> Result<(), Error<E>> {
        self.i2c.write(self.dev_addr, payload).map_err(Error::I2C)    
    }


    /*

    DOESN'T SEEM TO WORK - NEED TO TEST MORE

    /// Read data from the I2C bus
    fn read(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
        .write_read(self.dev_addr, &[register], &mut data)
        .map_err(Error::I2C)
        .and(Ok(data[0]))
    }

    /// Read the firmware revision number (currently 1)
    pub fn get_firmware_rev(&mut self) -> Result<u8, Error<E>> {
        let data = self.read(Register::FIRMWARE_REV)?;
        Ok(data)
    }

    /// Read the number of digits
    pub fn get_number_digits(&mut self) -> Result<u8, Error<E>> {
        let data = self.read(Register::NUMBER_DIGITS)?;
        Ok(data)
    }
    
     */


    /// Clear the display
    pub fn clear_display(&mut self) -> Result<(), Error<E>> {
        self.write(&[Register::CLEAR_DISPLAY])?;
        Ok(())
    }

    

    // NEED TO TEST MORE: TRIED WITH VALUE 0x69, CORRECTLY DISPLAYED A105 ON POWER-UP
    // BUT DID NOT RESPOND ON 0x69 I2C ADDRESS, RESPONDED ONLY WITH ADDRESS 0
    // SEEMS TO WORK OK UP TO 0x39
    // -- USE ADDRESS 0x00 TO RESET IN CASE OF PROBLEMS

    /// Set I2C address, defaults to 0x12
    pub fn set_address(&mut self, address: u8) -> Result<(), Error<E>> {
        //let mut dev_address = DEFAULT_ADDRESS;
        match address {
            //a if a < 0x7f => self.write(&[Register::I2C_ADDRESS_SETTING, a])?,
            a if a < 0x40 => self.write(&[Register::I2C_ADDRESS_SETTING, a])?,
            _ => (),
            }
        Ok(())
        }
    
    

    /// Show the current I2C address on the display
    pub fn display_address(&mut self) -> Result<(), Error<E>> {
        self.write(&[Register::DISPLAY_ADDRESS])?;
        Ok(())    
    }
    
    /// Set dsiplay brightness (0 - 255, 127 is 50%)
    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Error<E>> {        
        self.write(&[Register::BRIGHTNESS_SETTING, brightness])?;
        Ok(())        
    }

    /// Send a digit to the display without specifying the position
    pub fn send_digit(&mut self, number: u8) -> Result<(), Error<E>> {        
        if number > 9 {
            return Err(Error::InvalidInputData)
        } else {            
            self.write(&[number])?
        };
        Ok(())
    }

    /// Write digit D at position P
    pub fn display_digit(&mut self, position: u8, digit: u8) -> Result<(), Error<E>> {
        

        // TO DO: include hex digits:
        // 0x00 - 0x0f: Displays a single digit 0-9 or hexadecimal digit A-F.

        if position > 3 ||
           digit > 9 {
            return Err(Error::InvalidInputData);
            } else {                
                self.write(&[Register::POSITION_SETTING, position, digit])?
            };
        
        Ok(())

    }

   
    /// Display a number using all four digits
    /// TO DO: ADD A BOOLEAN SWITCH "with_leading_zeros"
    pub fn display_number(&mut self, number: u16) -> Result<(), Error<E>> {
        
        if number > 9999 {
            return Err(Error::InvalidInputData);
        } 

        let digits = TWIDisplay::<I2C>::get_digits(number);
        
        for (idx, digit) in digits.iter().enumerate() {
            self.display_digit(idx as u8, *digit)?
        }       

        Ok(())

    }
    
    
    /// Send a character to the display without specifying the position
    pub fn send_char(&mut self, ch: char) -> Result<(), Error<E>> {        
        
        // TO DO: restrict to 0x0g - 0x79
                
        self.write(&[ch as u8])?;        
        Ok(())
    }
 
    
    /// Write character C at position P
    pub fn display_char(&mut self, position: u8, ch: char) -> Result<(), Error<E>> {        
        
        // TO DO: restrict to 0x0g - 0x79
        
        if position > 3 {
            return Err(Error::InvalidInputData);            
            } else {              
                self.write(&[Register::POSITION_SETTING, position, ch as u8])?;
           };        
        Ok(())
    }

    /// Send text to the display
    pub fn send_text(&mut self, text: &str) -> Result<(), Error<E>> {
        for ch in text.chars() {
            self.send_char(ch)?
        }
        Ok(())
    }

    /// Display time in HH:MM format, with an optional dot between them
    pub fn display_time(&mut self, hours: u8, minutes: u8, dot: bool) -> Result<(), Error<E>> {
                
        if hours > 23 || minutes > 59 {
            return Err(Error::InvalidInputData)
        } else {            
            
            let time_value = (hours as u16) * 100 + minutes as u16;
            
            self.display_number(time_value)?

        };
        
        match dot {
            true => self.write(&[Register::DOTS, BitFlags::DOT2])?, // dot at second position
            false => self.write(&[Register::DOTS, 0b0000_0000])?,
        }
        
        Ok(())

    }


    /// Set the display mode: Scroll or Rotate (see documentation)
    pub fn set_mode(&mut self, mode: Mode) -> Result<(), Error<E>> {
        
        match mode {
            Mode::Rotate => self.write(&[Register::MODE_SETTING, 0])?,
            Mode::Scroll => self.write(&[Register::MODE_SETTING, 1])?,
        }        
        Ok(())
    }

    /// Display data with units (temperature, humidity) and defined thresholds
    fn display_data(&mut self, 
                    data: i16, unit: char, 
                    lo_thresh: Option<i16>, hi_thresh: Option<i16>, 
                    min_val: i16, max_val: i16) -> Result<(), Error<E>> {

        // check if limits can be accepted, if not reset to -99/999                            
        if min_val < (-99) || max_val > 999 {
            let (min_val, max_val): (i16,i16) = (-99, 999);
        }

        // thresholds initialized as min/max limits
        let mut lo_th: i16 = min_val; 
        let mut hi_th: i16 = max_val;

        match lo_thresh {
            Some(val) => lo_th = val, // if lower threshold was given
            None => lo_th = min_val,
        }

        match hi_thresh {
            Some(val) => hi_th = val, // if upper threshold was given
            None => lo_th = max_val,
        }

        // display -LL- and -HH- for data exceding thresholds, 
        // e.g. -20 and +50 for a temperature sensor
        
        if data < min_val || data > max_val {
            for (pos,ch) in "----".chars().enumerate() {
                self.display_char(pos as u8, ch)?
                
            }    
        } else if data < lo_th {
            for (pos,ch) in "-LL-".chars().enumerate() {
                self.display_char(pos as u8, ch)?
                
            }    
            
        } else if data > hi_th {
            for (pos,ch) in "-HH-".chars().enumerate() {                
                self.display_char(pos as u8, ch)?
                
            }    
            
        } else {
        
            let hundreds: u8 = (data.abs() / 100) as u8;
            let decimals: u8 = ((data.abs() % 100) / 10) as u8; 
           
            // position 0 (hundreds or minus sign)
            if data < 0 {
                //self.write(&[Register::POSITION_SETTING, 0, '-' as u8])?
                self.display_char(0, '-')?
            } else if hundreds == 0 {
                self.display_char(0, ' ')?
                //self.write(&[Register::POSITION_SETTING, 0, ' ' as u8])?
                
            } else {
                self.display_digit(0, hundreds)?                             
            }

            // position 1 (decimals)
            if (hundreds == 0 || data < 0) && decimals == 0 {
                self.display_char(1, ' ')?
                //self.write(&[Register::POSITION_SETTING, 1, ' ' as u8])?
            } else {              
                self.display_digit(1, decimals)?
            }

            // position 2 
            //self.write(&[Register::POSITION_SETTING, 2, (data.abs()  % 10) as u8])?;
            self.display_digit(2, (data.abs() % 10) as u8)?;

            // position 3 (unit)
            //self.write(&[Register::POSITION_SETTING, 3, unit as u8])?;
            self.display_char(3, unit)?;

        }

        Ok(())

    }

    /// Display temperature between -99 and 999 with a chosen unit, with lower and upper threshold
    
    pub fn display_temperature(&mut self, temperature: i16, unit: TempUnits, lo_thresh: Option<i16>, hi_thresh: Option<i16>) -> Result<(), Error<E>> {
        
        let mut temp_unit = 'C';       
        let (min_val, max_val): (i16,i16) = (-99, 999);

        let mut lo_th: i16 = min_val;
        let mut hi_th: i16 = max_val;

        match unit {
            TempUnits::Celsius => temp_unit = 'C',
            TempUnits::Fahrenheit => temp_unit = 'F',
        }
        
        match lo_thresh {
            Some(th) => lo_th = th,
            None => lo_th = min_val,
        }

        if lo_th < min_val {
            lo_th = min_val
        }

        match hi_thresh {
            Some(th) => hi_th = th,
            None => hi_th = max_val,
        }

        if hi_th > max_val {
            hi_th = max_val
        }

        self.display_data(temperature, temp_unit, Some(lo_th), Some(hi_th), min_val, max_val)?;

        Ok(())

    }

    /// Display humidity in range 0-100, with lower and upper threshold. 

    pub fn display_humidity(&mut self, humidity: i16, lo_thresh: Option<i16>, hi_thresh: Option<i16>) -> Result<(), Error<E>> {
                
        let (min_val, max_val): (i16,i16) = (0, 100);

        let mut lo_th: i16 = min_val;
        let mut hi_th: i16 = max_val;
        
        match lo_thresh {
            Some(th) => lo_th = th,
            None => lo_th = min_val,
        }

        if lo_th < min_val {
            lo_th = min_val
        }

        match hi_thresh {
            Some(th) => hi_th = th,
            None => hi_th = max_val,
        }

        if hi_th > max_val {
            hi_th = max_val
        }
        self.display_data(humidity, 'H', Some(lo_th), Some(hi_th), min_val, max_val)?;

        Ok(())

    }


    /// Get digits from a 4-digit number
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

