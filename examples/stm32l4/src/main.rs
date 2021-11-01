/* 
Test various functions of the Akafugu TWIDisplay
*/

#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32l4xx_hal::{
    delay::Delay,
    prelude::*,    
    i2c::I2c,
    };

use akafugu_twidisplay::*;

const BOOT_DELAY_MS: u16 = 100; 

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);  
    
    let mut delay = Delay::new(cp.SYST, clocks);

    // delay necessary for the I2C to initiate correctly and start on boot without having to reset the board
    delay.delay_ms(BOOT_DELAY_MS);
    
    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);
    
    let mut akafugu = TWIDisplay::new(i2c, DEFAULT_ADDRESS);
    
    akafugu.clear_display().unwrap();

    loop {
        
        // demo sequence

        // maximum brightness
        akafugu.set_brightness(255).unwrap();

        led.set_high().ok();

        // show the current I2C address
        akafugu.display_address().unwrap(); 

        delay.delay_ms(1000_u32);       

        led.set_low().ok();

        // display temperature with Celsius units, and set lower and upper thresholds
        akafugu.display_temperature(-10,TempUnits::Celsius, Some(-50), Some(199)).unwrap(); 
    
        delay.delay_ms(1000_u32);

        led.set_high().ok();
        
        // display time with the dot on
        akafugu.display_time(9,7, true).unwrap();            

        delay.delay_ms(1000_u32);

        led.set_low().ok();
    
        // minimmum brightness
        akafugu.set_brightness(0).unwrap();

        // display time with the dot off
        akafugu.display_time(17,0, false).unwrap();
    
        delay.delay_ms(1000_u32);
    
        led.set_high().ok();

        // display humidity, no thresholds
        akafugu.display_humidity(65, None, None).unwrap();
    
        delay.delay_ms(1000_u32);
        
        akafugu.clear_display().unwrap();
        
        // 50% brightness
        akafugu.set_brightness(127).unwrap();
    
        led.set_low().ok();

        // display temperature in Fahrenheits; as the lower threshold is set to 0, 
        // it will show as -LL-
        akafugu.display_temperature(-25, TempUnits::Fahrenheit, Some(0), None).unwrap();
        
        delay.delay_ms(1000 as u32);    

        led.set_high().ok();

        // display temperature that is below the display limits (-99),
        // it will show as '----'
        akafugu.display_temperature(-110, TempUnits::Fahrenheit, None, None).unwrap();
        
        delay.delay_ms(1000 as u32);    

        led.set_low().ok();

        // turn first and third dot on; previous '----' stays on
        akafugu.display_dots([true, false, true, false]).unwrap();

        delay.delay_ms(1000 as u32);    

        led.set_high().ok();

        // clean up the display
        akafugu.clear_display().unwrap();

        // turn second and fourth dot on        
        akafugu.display_dots([false, true, false, true]).unwrap();

        delay.delay_ms(1000 as u32);    

        led.set_high().ok();

        }
    
    
}

